import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function samplePanui(cell: CallableCell, partialPanui = {}) {
    return {
        ...{
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialPanui
    };
}

export async function createPanui(cell: CallableCell, panui = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "panui",
      fn_name: "create_panui",
      payload: panui || await samplePanui(cell),
    });
}



export async function sampleKatoitoi(cell: CallableCell, partialKatoitoi = {}) {
    return {
        ...{
	  katoitoi: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
          panui_hash: (await createPanui(cell)).signed_action.hashed.hash,
        },
        ...partialKatoitoi
    };
}

export async function createKatoitoi(cell: CallableCell, katoitoi = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "panui",
      fn_name: "create_katoitoi",
      payload: katoitoi || await sampleKatoitoi(cell),
    });
}

