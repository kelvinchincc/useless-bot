/***********************************************************************************************************************
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 **********************************************************************************************************************/
import { CommandDescriptor } from '../../constants/commands';
import { logger } from '../../logger';
import { pb } from '../../store/pbstore';
import { CommandReducer, MessageContextMenuCommandReducer } from '../command';
import {
    ActionRowBuilder,
    ApplicationCommandOptionType,
    ApplicationCommandType,
    ModalBuilder,
    TextInputBuilder,
    TextInputStyle,
} from 'discord.js';
import { v4 as uuid } from 'uuid';

async function fetchSticker(key: string) {
    try {
        const sticker = await pb.collection('stickers').getFirstListItem(`key="${key}"`);
        return sticker == null ? '' : sticker.url;
    } catch (e) {
        return '';
    }
}

export const replyWithSticker: CommandReducer = async interaction => {
    const sticker = interaction.options.getString('sticker');
    const message = interaction.options.getString('message')!;

    logger.debug(`Replying with sticker: ${sticker} to message: ${message}`);

    // https://discord.com/channels/server_id/channel_id/message_id
    const messageParts = message.split('/');
    const targetMessageServerID = messageParts[4];
    const targetMessageChannelID = messageParts[5];
    const targetMessageID = messageParts[6];

    logger.debug(
        `Server ID: ${targetMessageServerID}, Channel ID: ${targetMessageChannelID}, Message ID: ${targetMessageID}`
    );

    const server = interaction.guild;

    if (!sticker) {
        await interaction.reply({
            content: 'No sticker provided.',
            ephemeral: true,
        });
        return;
    }

    const stickerUrl = await fetchSticker(sticker);
    if (!stickerUrl) {
        await interaction.reply({
            content: 'Sticker not found',
            ephemeral: true,
        });
        return;
    }

    if (interaction.channel?.isSendable()) {
        await interaction.channel.send({
            content: `[sticker](${stickerUrl})\nTriggered by <@${interaction.user.id}>`,
            allowedMentions: { parse: [], repliedUser: true },
            reply: {
                messageReference: targetMessageID,
            },
        });
    } else {
        await interaction.reply({
            content: 'Cannot send message to this channel',
            ephemeral: true,
        });
    }

    await interaction.reply({
        content: 'Sticker replied',
        ephemeral: true,
    });
};

export const replyWithStickerCommandDescription: CommandDescriptor = {
    name: 'reply',
    description: 'Reply with a sticker to seleted message',
    type: ApplicationCommandOptionType.Subcommand,
    options: [
        {
            name: 'sticker',
            description: 'The sticker to preview, only you can see the message.',
            type: ApplicationCommandOptionType.String,
            required: true,
            autocomplete: true,
        },
        {
            name: 'message',
            description: 'The message where should be replied to',
            type: ApplicationCommandOptionType.String,
            required: true,
            autocomplete: false,
        },
    ],
};
