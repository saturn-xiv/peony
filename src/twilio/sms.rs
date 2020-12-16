use super::super::errors::Result;
use super::Config;

impl Config {
    fn send(&self, _to: &str, _body: &str) -> Result<()> {
        // TODO
        Ok(())
    }
    fn recived(&self) -> Result<Vec<String>> {
        let items = Vec::new();
        Ok(items)
    }
}
