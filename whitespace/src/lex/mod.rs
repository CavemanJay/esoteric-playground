mod manual;
#[cfg(feature = "nom")]
mod nom;
#[cfg(feature = "pest")]
#[path = "pest/pest.rs"]
mod pest;

pub use manual::tokenize as tokenize_handrolled;
#[cfg(feature = "nom")]
pub use nom::tokenize as tokenize_with_nom;
#[cfg(feature = "pest")]
pub use pest::visible::tokenize as tokenize_with_pest_visible;
#[cfg(feature = "pest")]
pub use pest::invisible::tokenize as tokenize_with_pest;
