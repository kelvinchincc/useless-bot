/***********************************************************************************************************************
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 **********************************************************************************************************************/
import { AutocompleteInteraction } from 'discord.js';
import { logger } from '../logger';
import { sticker } from './sticker';

export type AutocompleteReducer = (interaction: AutocompleteInteraction) => Promise<void>;

const autocompleteReducersJumpTable: Record<string, AutocompleteReducer> = {
    sticker,
};

export async function invokeAutocomplete(command: string, interaction: AutocompleteInteraction) {
    logger.debug(`Autocomplete command: ${command}`);
    const reducer = autocompleteReducersJumpTable[command];

    if (reducer == null) {
        throw new Error(`No reducer found for command ${command}`);
    }

    try {
        return await reducer(interaction);
    } catch (error) {
        return logger.error(`Module autocomplete [${command}]: ${error}`);
    }
}
