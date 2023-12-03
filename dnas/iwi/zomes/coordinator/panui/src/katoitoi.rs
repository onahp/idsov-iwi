use hdk::prelude::*;
use panui_integrity::*;
#[hdk_extern]
pub fn create_katoitoi(katoitoi: Katoitoi) -> ExternResult<Record> {
    let katoitoi_hash = create_entry(&EntryTypes::Katoitoi(katoitoi.clone()))?;
    create_link(
        katoitoi.panui_hash.clone(),
        katoitoi_hash.clone(),
        LinkTypes::PanuiToKatoitois,
        (),
    )?;
    let record = get(katoitoi_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Katoitoi"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_katoitoi(katoitoi_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(katoitoi_hash, GetOptions::default())
}
#[hdk_extern]
pub fn delete_katoitoi(original_katoitoi_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_katoitoi_hash)
}
#[hdk_extern]
pub fn get_katoitois_for_panui(panui_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let links = get_links(panui_hash, LinkTypes::PanuiToKatoitois, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
