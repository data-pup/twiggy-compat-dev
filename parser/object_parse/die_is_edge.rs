use gimli;
use traits;

pub fn is_edge<R>(
    _die: &gimli::DebuggingInformationEntry<R, R::Offset>,
) -> Result<bool, traits::Error>
where
    R: gimli::Reader,
{
    unimplemented!();
}
