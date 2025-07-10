use tnr::{Notify, Type};

pub struct TnrWrapper;

impl TnrWrapper {
    pub fn new() -> Self {
        Self
    }

    pub fn notify(&self, title: &str, message: &str, notification_type: Option<&str>, url: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let mut notify = Notify::new(message)
            .with_title(title);

        if let Some(type_str) = notification_type {
            if let Some(tnr_type) = Type::from_str(type_str) {
                notify = notify.with_type(tnr_type);
            }
        }

        if let Some(open_url) = url {
            notify = notify.with_url(open_url);
        }

        notify.send()?;
        
        Ok(())
    }
}

impl Default for TnrWrapper {
    fn default() -> Self {
        Self::new()
    }
}