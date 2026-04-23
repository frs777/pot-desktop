import { Divider, Button, Popover, PopoverTrigger, PopoverContent, Tooltip } from '@nextui-org/react';
import { appLogDir, appConfigDir } from '@tauri-apps/api/path';
import { useTranslation } from 'react-i18next';
import { open } from '@tauri-apps/plugin-shell';
import { BsTelegram } from 'react-icons/bs';
import { BsGithub } from 'react-icons/bs';
import React from 'react';

import { appVersion } from '../../../../utils/env';

export default function About() {
    const { t } = useTranslation();

    return (
        <div className='h-full w-full py-[80px] px-[100px]'>
            <img
                src='icon.png'
                className='mx-auto h-[100px] mb-[5px]'
                draggable={false}
            />
            <div className='content-center'>
                <h1 className='font-bold text-2xl text-center'>Pot-F</h1>
                <p className='text-center text-sm text-gray-500 mb-[5px]'>{appVersion}</p>
                <p className='text-center text-xs text-gray-400 mb-[10px]'>
                    {t('config.about.fork_info')}
                </p>
                <Divider />
                <div className='flex justify-between'>
                    <Button
                        variant='light'
                        className='my-[5px]'
                        size='sm'
                        onPress={() => {
                            open('https://frs777.github.io/pot-f-desktop');
                        }}
                    >
                        {t('config.about.website')}
                    </Button>
                    <Button
                        variant='light'
                        className='my-[5px]'
                        size='sm'
                        onPress={() => {
                            open('https://github.com/frs777/pot-f-desktop');
                        }}
                    >
                        {t('config.about.github')}
                    </Button>
                    <Popover
                        placement='top'
                        offset={10}
                    >
                        <PopoverTrigger>
                            <Button
                                variant='light'
                                className='my-[5px]'
                                size='sm'
                            >
                                {t('config.about.feedback')}
                            </Button>
                        </PopoverTrigger>
                        <PopoverContent>
                            <div className='flex justify-between'>
                                <Button
                                    variant='light'
                                    className='my-[5px]'
                                    size='sm'
                                    onPress={() => {
                                        open('https://github.com/frs777/pot-f-desktop/issues');
                                    }}
                                >
                                    {t('config.about.issue')}
                                </Button>
                                <Button
                                    variant='light'
                                    className='my-[5px]'
                                    size='sm'
                                    onPress={() => {
                                        open('mailto:frss@protonmail.com');
                                    }}
                                >
                                    {t('config.about.email')}
                                </Button>
                            </div>
                        </PopoverContent>
                    </Popover>

                    <Popover
                        placement='top'
                        offset={10}
                    >
                        <PopoverTrigger>
                            <Button
                                variant='light'
                                className='my-[5px]'
                                size='sm'
                            >
                                {t('config.about.community')}
                            </Button>
                        </PopoverTrigger>
                        <PopoverContent>
                            <div className='flex justify-between'>
                                <Tooltip content={t('config.about.telegram')}>
                                    <Button
                                        isIconOnly
                                        variant='light'
                                        className='my-[5px]'
                                        size='lg'
                                        onPress={() => {
                                            open('https://t.me/pot_app');
                                        }}
                                    >
                                        <BsTelegram />
                                    </Button>
                                </Tooltip>
                                <Tooltip content={t('config.about.discussion')}>
                                    <Button
                                        isIconOnly
                                        variant='light'
                                        className='my-[5px]'
                                        size='lg'
                                        onPress={() => {
                                            open('https://github.com/frs777/pot-f-desktop/discussions');
                                        }}
                                    >
                                        <BsGithub />
                                    </Button>
                                </Tooltip>
                            </div>
                        </PopoverContent>
                    </Popover>
                </div>
                <Divider />
            </div>
            <div className='content-center px-[40px]'>
                <div className='flex justify-between'>
                    <Button
                        variant='light'
                        className='my-[5px]'
                        size='sm'
                        onPress={() => {
                            open('https://github.com/frs777/pot-f-desktop/releases');
                        }}
                    >
                        {t('config.about.check_update')}
                    </Button>
                    <Button
                        variant='light'
                        className='my-[5px]'
                        size='sm'
                        onPress={async () => {
                            const dir = await appLogDir();
                            open(dir);
                        }}
                    >
                        {t('config.about.view_log')}
                    </Button>
                    <Button
                        variant='light'
                        className='my-[5px]'
                        size='sm'
                        onPress={async () => {
                            const dir = await appConfigDir();
                            open(dir);
                        }}
                    >
                        {t('config.about.view_config')}
                    </Button>
                </div>

                <Divider />
            </div>
        </div>
    );
}
