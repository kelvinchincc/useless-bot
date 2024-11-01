/***********************************************************************************************************************
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 **********************************************************************************************************************/
import { CommandReducer } from '../command';
import { pb } from '../../store/pbstore';
import { StickerCollection } from '../../dto/sticker';
import { CommandDescriptor } from '../../constants/commands';
import { ApplicationCommandOptionType } from 'discord.js';

export const send: CommandReducer = async interaction => {
    const targetSticker = interaction.options.getString('sticker');
    const quote = interaction.options.getString('quote');

    try {
        const sticker = await pb.collection<StickerCollection>('stickers').getFirstListItem(`key="${targetSticker}"`);
        if (quote) {
            await interaction.reply(`${quote}\n[sticker](${sticker.url})`);
        } else {
            await interaction.reply(sticker.url);
        }
    } catch (error) {
        await interaction.reply({ content: `Sticker ${targetSticker} not found`, ephemeral: true });
    }
};

export const sendCommandDescription: CommandDescriptor = {
    name: 'send',
    description: 'Send a sticker',
    type: ApplicationCommandOptionType.Subcommand,
    options: [
        {
            name: 'sticker',
            description: 'The sticker to send',
            type: ApplicationCommandOptionType.String,
            required: true,
            autocomplete: true,
        },
        {
            name: 'quote',
            description: 'Message to append to the sticker',
            type: ApplicationCommandOptionType.String,
            required: false,
        },
    ],
};
