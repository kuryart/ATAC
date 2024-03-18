use std::sync::Arc;
use crate::app::app::App;
use crate::app::app_states::AppState;
use crate::app::ui::param_tabs::param_tabs::RequestParamsTabs;
use crate::request::body::ContentType;
use crate::request::request::KeyValue;

impl App<'_> {
    pub fn normal_state(&mut self) {
        self.state = AppState::Normal;
    }
    
    pub fn display_cookies_state(&mut self) {
        let local_cookie_store = Arc::clone(&self.cookies_popup.cookie_store);

        self.cookies_popup.cookies_table.rows = vec![];

        for cookie in local_cookie_store.read().unwrap().iter_any() {
            let (name, value) = cookie.name_value();

            self.cookies_popup.cookies_table.rows.push(KeyValue {
                enabled: true,
                data: (name.to_string(), value.to_string()),
            })
        }

        self.update_cookies_table_selection();
        self.state = AppState::DisplayingCookies;
    }

    pub fn edit_cookie_state(&mut self) {
        let selection = self.cookies_popup.cookies_table.selection.unwrap();

        let input_text = match selection {
            (x, 0) => self.cookies_popup.cookies_table.rows[x].data.0.clone(),
            (x, 1) => self.cookies_popup.cookies_table.rows[x].data.1.clone(),
            _ => String::new() // Should not happen
        };

        self.cookies_popup.cookies_table.selection_text_input.reset_input();
        self.cookies_popup.cookies_table.selection_text_input.enter_str(&input_text);
        self.cookies_popup.cookies_table.selection_text_input.cursor_position = input_text.len();

        self.state = AppState::EditingCookies;
    }

    pub fn create_new_collection_state(&mut self) {
        self.state = AppState::CreatingNewCollection;
    }

    pub fn create_new_request_state(&mut self) {
        let collections_length = self.collections.len();

        // Cannot create a request if there is no collection
        if collections_length == 0 {
            return;
        }

        self.new_request_popup.max_selection = collections_length;
        self.state = AppState::CreatingNewRequest;
    }

    pub fn delete_collection_state(&mut self) {
        self.delete_collection_popup.state = false;
        self.state = AppState::DeletingCollection;
    }

    pub fn delete_request_state(&mut self) {
        self.delete_request_popup.state = false;
        self.state = AppState::DeletingRequest;
    }

    pub fn rename_collection_state(&mut self) {
        let selected_request_index = self.collections_tree.state.selected();

        let collection_name = &self.collections[selected_request_index[0]].name;
        self.rename_collection_input.text = collection_name.clone();
        self.rename_collection_input.cursor_position = collection_name.len();
        
        self.state = AppState::RenamingCollection;
    }

    pub fn rename_request_state(&mut self) {
        let selected_request_index = self.collections_tree.state.selected();

        {
            let selected_request = self.collections[selected_request_index[0]].requests[selected_request_index[1]].read().unwrap();
            self.rename_request_input.text = selected_request.name.clone();
            self.rename_request_input.cursor_position = selected_request.name.len();
        }

        self.state = AppState::RenamingRequest;
    }
    
    pub fn select_request_state(&mut self) {
        self.state = AppState::SelectedRequest;
        self.update_inputs();
    }

    pub fn edit_request_url_state(&mut self) {
        self.state = AppState::EditingRequestUrl;
    }

    pub fn edit_request_param_state(&mut self) {
        self.state = AppState::EditingRequestParam;
        self.update_inputs();
    }

    pub fn edit_request_auth_username_state(&mut self) {
        self.state = AppState::EditingRequestAuthUsername;
    }

    pub fn edit_request_auth_password_state(&mut self) {
        self.state = AppState::EditingRequestAuthPassword;
    }

    pub fn edit_request_auth_bearer_token_state(&mut self) {
        self.state = AppState::EditingRequestAuthBearerToken;
    }

    pub fn edit_request_header_state(&mut self) {
        self.state = AppState::EditingRequestHeader;
        self.update_inputs();
    }

    pub fn edit_request_body_table_state(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let selected_request = local_selected_request.read().unwrap();

            match selected_request.body {
                ContentType::Multipart(_) | ContentType::Form(_) => {}
                _ => {
                    return;
                }
            }
        }

        self.request_param_tab = RequestParamsTabs::Body;
        self.state = AppState::EditingRequestBodyTable;
        self.update_inputs();
    }


    pub fn edit_request_body_string_state(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let selected_request = local_selected_request.read().unwrap();

            match selected_request.body {
                ContentType::Raw(_) | ContentType::Json(_) | ContentType::Xml(_) | ContentType::Html(_) => {}
                _ => {
                    return;
                }
            }
        }

        self.request_param_tab = RequestParamsTabs::Body;
        self.state = AppState::EditingRequestBodyString;
    }

    pub fn edit_request_settings_state(&mut self) {
        self.request_settings_popup.selection = 0;

        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read().unwrap();

        self.request_settings_popup.settings = selected_request.settings.to_vec();

        self.state = AppState::EditingRequestSettings;
    }
}