use crate::{
    preprocessors::FrontMatter,
    utils::{find_zk_root, update_summary},
};
use mdbook::{errors::Error, Config, MDBook};
use mdbook_backlinks::Backlinks;
use mdbook_katex::KatexProcessor;
use mdbook_wikilink::WikiLinks;
use std::path::PathBuf;

pub fn build(dir: Option<PathBuf>) -> Result<(), Error> {
    let zk = init_zk(dir)?;

    zk.build().expect("Builing failed");

    Ok(())
}

pub fn init_zk(dir: Option<PathBuf>) -> Result<MDBook, Error> {
    let path = match dir {
        Some(path) => path,
        None => find_zk_root().ok_or(Error::msg("Could not find the root of your Zettelkasten"))?,
    };

    let config_path: PathBuf = path.join("zk.toml");
    let config: Config = Config::from_disk(config_path)?;

    let book_source = &config.book.src;
    update_summary(&book_source)?;

    let mut zk = MDBook::load_with_config(path, config)?;
    zk.with_preprocessor(FrontMatter);
    zk.with_preprocessor(KatexProcessor);
    zk.with_preprocessor(Backlinks);
    zk.with_preprocessor(WikiLinks);

    Ok(zk)
}
