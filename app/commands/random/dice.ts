/***********************************************************************************************************************
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 **********************************************************************************************************************/
import { ApplicationCommandOptionType } from 'discord.js';
import { CommandDescriptor } from '../../constants/commands';
import { CommandReducer } from '../command';
import { logger } from '../../logger';

const diceNumberEmojiMapping: Record<number, string> = {
    1: ':one:',
    2: ':two:',
    3: ':three:',
    4: ':four:',
    5: ':five:',
    6: ':six:',
    7: ':seven:',
    8: ':eight:',
    9: ':nine:',
    0: ':zero:',
};

const diceVariant: Record<string, number> = {
    '4 Faces': 4,
    '6 Faces': 6,
    '8 Faces': 8,
    '10 Faces': 10,
    '12 Faces': 12,
    '24 Faces': 24,
};

export const dice: CommandReducer = async interaction => {
    const variant = Number.parseInt(interaction.options.getString('variant') ?? '6');
    const roll = Math.floor(Math.random() * variant) + 1;

    logger.debug('random dice: ', { variant, roll });

    if (variant >= 10) {
        await interaction.reply(`# **${roll}**`);
        return;
    }

    await interaction.reply({ content: diceNumberEmojiMapping[`${roll}`] });
};

export const diceCommandDescription: CommandDescriptor = {
    name: 'dice',
    description: 'Roll a dice',
    type: ApplicationCommandOptionType.Subcommand,
    options: [
        {
            name: 'variant',
            description: 'The number of faces of the dice',
            type: ApplicationCommandOptionType.String,
            required: false,
            choices: Object.entries(diceVariant).map(([name, value]) => ({ name, value: `${value}` })),
        },
    ],
};
