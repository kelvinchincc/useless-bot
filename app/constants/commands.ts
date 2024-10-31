/***********************************************************************************************************************
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 **********************************************************************************************************************/
import { ApplicationCommandOptionType, ApplicationCommandType } from 'discord.js';
import { CommandReducer, MessageContextMenuCommandReducer } from '../commands/command';
import { ping, pingCommandDescription } from '../commands/ping';
import { sticker, stickerCommandDescription } from '../commands/sticker';
import { about, aboutCommandDescription } from '../commands/about';
import { random, randomCommandDescription } from '../commands/random';
import { replyWithSticker, replyWithStickerCommandDescription } from '../commands/sticker/replyWithSticker';
import { letMeGoogleItForYou, letMeGoogleItForYouCommandDescription } from '../commands/letMeGoogleItForYou';

/**
 * Represented as the body of application commands that will be sent to Discord.
 */
export type CommandDescriptor = Record<string, string | number | boolean | CommandDescriptor[]>;

export const commands: CommandDescriptor[] = [
    { ...pingCommandDescription },
    { ...aboutCommandDescription },
    { ...randomCommandDescription },
    { ...stickerCommandDescription },
    { ...letMeGoogleItForYouCommandDescription },
];

export const commandJumpTable: Record<string, CommandReducer> = {
    ping,
    sticker,
    about,
    random,
    'let-me-google-it-for-you': letMeGoogleItForYou,
};

export const messageContextMenuCommandJumpTable: Record<string, MessageContextMenuCommandReducer> = {
    // 'Reply with sticker': replyWithSticker,
};
