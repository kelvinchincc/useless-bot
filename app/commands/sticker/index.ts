/***********************************************************************************************************************
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 **********************************************************************************************************************/
import { CommandDescriptor } from '../../constants/commands';
import { logger } from '../../logger';
import { pb } from '../../store/pbstore';
import { execIfNotProd } from '../../utils/functional';
import { CommandReducer } from '../command';
import { preview, previewCommandDescription } from './preview';
import { replyWithSticker, replyWithStickerCommandDescription } from './replyWithSticker';
import { send, sendCommandDescription } from './send';
import { simpleList, simpleListCommandDescription } from './simple_list';

const subcommandJumpTable: Record<string, CommandReducer> = {
    'simple-list': simpleList,
    preview,
    send,
    reply: replyWithSticker,
};

export const sticker: CommandReducer = async interaction => {
    const subcommand = interaction.options.getSubcommand();
    execIfNotProd(() => logger.debug(`Executing subcommand ${subcommand}`));

    if (!(subcommand in subcommandJumpTable)) {
        logger.warn(`Subcommand ${subcommand} not found in the registry.`);
        return;
    }

    try {
        await subcommandJumpTable[subcommand](interaction);
    } catch (e) {
        logger.error('Error in module "sticker": ', e);
    }
};

export const stickerCommandDescription: CommandDescriptor = {
    name: 'sticker',
    description: 'Replies with a sticker!',
    options: [
        { ...simpleListCommandDescription },
        { ...previewCommandDescription },
        { ...sendCommandDescription },
        { ...replyWithStickerCommandDescription },
    ],
};
