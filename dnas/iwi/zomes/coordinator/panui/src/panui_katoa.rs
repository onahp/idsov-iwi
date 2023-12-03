use hdk::prelude::*;
use panui_integrity::*;
#[hdk_extern]
pub fn get_panui_katoa(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("panui_katoa");
    let links = get_links(path.path_entry_hash()?, LinkTypes::PanuiKatoa, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}
