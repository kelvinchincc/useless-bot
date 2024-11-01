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
    ChatInputCommandInteraction,
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

interface TargetMessageInfo {
    serverID: string;
    channelID: string;
    messageID: string;
}

function isSameServerAndChannel(intearction: ChatInputCommandInteraction, targetMessage: TargetMessageInfo) {
    const { guild, channel } = intearction;
    logger.debug('Guild: ', guild?.id, 'Channel: ', channel?.id, 'Target Message: ', targetMessage);

    if (guild?.id !== targetMessage.serverID) {
        return false;
    }

    if (channel?.id !== targetMessage.channelID) {
        return false;
    }

    return true;
}

export const replyWithSticker: CommandReducer = async interaction => {
    const sticker = interaction.options.getString('sticker');
    const message = interaction.options.getString('message-link')!;
    const quote = interaction.options.getString('quote');

    logger.debug(`Replying with sticker: ${sticker} to message: ${message}`);

    // https://discord.com/channels/server_id/channel_id/message_id
    const messageParts = message.split('/');
    const targetMessage: TargetMessageInfo = {
        serverID: messageParts[4],
        channelID: messageParts[5],
        messageID: messageParts[6],
    };

    logger.debug('Target Message: ', targetMessage);

    if (!isSameServerAndChannel(interaction, targetMessage)) {
        await interaction.reply({
            content: 'You can only reply to messages in the same server and channel.',
            ephemeral: true,
        });
        return;
    }

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
            content: `${quote ? `${quote}\n` : ''}[sticker](${stickerUrl})`,
            reply: {
                messageReference: targetMessage.messageID,
            },
        });
        await interaction.channel.send({
            content: `Triggered by <@${interaction.user.id}>`,
            allowedMentions: { parse: [], repliedUser: true },
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
            description: 'The sticker to reply',
            type: ApplicationCommandOptionType.String,
            required: true,
            autocomplete: true,
        },
        {
            name: 'message-link',
            description: 'The message where should be replied to, copy the Message Link and paste it here.',
            type: ApplicationCommandOptionType.String,
            required: true,
            autocomplete: false,
        },
        {
            name: 'quote',
            description: 'Message to append to the sticker',
            type: ApplicationCommandOptionType.String,
            required: false,
        },
    ],
};
