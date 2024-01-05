import {
    describeLitentry,
    buildIdentityTxs,
    buildIdentityHelper,
    buildValidations,
    checkErrorDetail,
    buildIdentityFromKeypair,
    PolkadotSigner,
} from './common/utils';
import { step } from 'mocha-steps';
import { sendTxsWithUtility } from './common/transactions';
import { generateWeb3Wallets, assertIdGraphMutationEvent, assertIdentityDeactivated } from './common/utils';
import { ethers } from 'ethers';
import type { LitentryValidationData, Web3Network, CorePrimitivesIdentity } from 'parachain-api';
import { Vec } from '@polkadot/types';

describeLitentry('Test Batch Utility', (context) => {
    let identities: CorePrimitivesIdentity[] = [];
    let validations: LitentryValidationData[] = [];
    let evmSigners: ethers.Wallet[] = [];
    const we3networks: Web3Network[][] = [];
    const signerIdentities: CorePrimitivesIdentity[] = [];

    step('generate web3 wallets', async function () {
        const web3Wallets = await generateWeb3Wallets(3);
        evmSigners = web3Wallets.map((web3Signer) => {
            return web3Signer.evmWallet;
        });
    });

    step('batch test: link identities', async function () {
        const defaultNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum']);
        const aliceSubstrateIdentity = await buildIdentityFromKeypair(
            new PolkadotSigner(context.substrateWallet.alice),
            context
        );

        for (let index = 0; index < evmSigners.length; index++) {
            const signer = evmSigners[index];
            const evmIdentity = await buildIdentityHelper(signer.address, 'Evm', context);
            identities.push(evmIdentity);
            we3networks.push(defaultNetworks as unknown as Vec<Web3Network>); // @fixme #1878
            signerIdentities.push(aliceSubstrateIdentity);
        }

        const evmValidations = await buildValidations(
            context,
            signerIdentities,
            identities,
            1,
            'ethereum',
            undefined,
            evmSigners
        );
        validations = [...evmValidations];

        const txs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            identities,
            'linkIdentity',
            validations,
            we3networks
        );
        const events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'identityManagement', [
            'IdentityLinked',
        ]);

        const identityLinkedEvents = events.filter((e) => context.api.events.identityManagement.IdentityLinked.is(e));
        await assertIdGraphMutationEvent(
            context,
            new PolkadotSigner(context.substrateWallet.alice),
            identityLinkedEvents,
            undefined,
            txs.length
        );
    });

    step('batch test: deactivate identities', async function () {
        const txs = await buildIdentityTxs(context, context.substrateWallet.alice, identities, 'deactivateIdentity');
        const deactivatedEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            txs,
            'identityManagement',
            ['IdentityDeactivated']
        );

        await assertIdentityDeactivated(context, new PolkadotSigner(context.substrateWallet.alice), deactivatedEvents);
    });

    step('batch test: deactivate error identities', async function () {
        identities = [];
        // prepare new identities that were not linked - so they do not exist
        for (let index = 0; index < evmSigners.length; index++) {
            const evmIdentity = await buildIdentityHelper('twitter_user_' + index, 'Twitter', context);
            identities.push(evmIdentity);
        }

        const txs = await buildIdentityTxs(context, context.substrateWallet.alice, identities, 'deactivateIdentity');
        const deactivatedEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            txs,
            'identityManagement',
            ['DeactivateIdentityFailed']
        );
        await checkErrorDetail(deactivatedEvents, 'IdentityNotExist');
    });

    step('check IDGraph after deactivateIdentity', async function () {
        // TODO: check the idgraph is empty
    });
});
