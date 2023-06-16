import "@polkadot/api/augment";
import "@polkadot/types/augment";
import { ApiOptions } from "@polkadot/api/types";
import { identity } from "../parachain-interfaces/definitions";

import { ApiPromise } from "@polkadot/api";

type ProviderInterface = Exclude<ApiOptions["provider"], undefined>;

export async function create(provider: ProviderInterface): Promise<ApiPromise> {
    return await ApiPromise.create({ provider, types: identity.types });
}
