pub enum Kind {
    SDTM,
    ADAM,
    TFL,
}

pub trait ConfigReader {
    fn read(&self) -> anyhow::Result<Vec<(String, bool)>>;
}
