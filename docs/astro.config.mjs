// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
        site: 'https://docs.forgedesk.eu.cc',
        integrations: [
                starlight({
                        title: 'ForgeDesk',
                        head: [
                                { tag: 'link', attrs: { rel: 'alternate', type: 'application/rss+xml', title: 'ForgeDesk Blog', href: 'https://docs.forgedesk.eu.cc/rss.xml' } }
                        ],
                        customCss: [
                                './src/styles/custom.css',
                        ],
                        logo: {
                                src: './src/assets/logo.svg',
                        },
                        description: 'Build small, fast, secure desktop apps with Python + native webviews.',
                        social: [
                                { icon: 'github', label: 'GitHub', href: 'https://github.com/swadhinbiswas/ForgeDesk' }
                        ],
                        components: {
                                SocialIcons: './src/components/SiteNavigation.astro',
                        },
                        sidebar: [
                                {
                                        label: 'Guides',
                                        autogenerate: { directory: 'guides' },
                                },
                                {
                                        label: 'References',
                                        autogenerate: { directory: 'references' },
                                        collapsed: true,
                                },
                                {
                                        label: 'Quick Start',
                                        autogenerate: { directory: 'quick-start' },
                                },
                                {
                                        label: 'Core Concepts',
                                        autogenerate: { directory: 'core-concepts' },
                                },
                                {
                                        label: 'Security',
                                        autogenerate: { directory: 'security' },
                                },
                                {
                                        label: 'Develop',
                                        autogenerate: { directory: 'develop' },
                                },
                                {
                                        label: 'Distribute',
                                        autogenerate: { directory: 'distribute' },
                                },
                                {
                                        label: 'Learn',
                                        autogenerate: { directory: 'learn' },
                                },
                                {
                                        label: 'Plugins',
                                        autogenerate: { directory: 'plugins' },
                                }
                        ],
                }),
        ],
});
