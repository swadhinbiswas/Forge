// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
integrations: [
starlight({
title: 'ForgeDesk',
social: [
{ icon: 'github', label: 'GitHub', href: 'https://github.com/swadhinbiswas/ForgeDesk' }
],
sidebar: [
{
label: 'Start Here',
items: [
{ label: 'Getting Started', slug: 'getting-started' },
{ label: 'Architecture', slug: 'architecture' },
{ label: 'Migration from Electron', slug: 'migration-from-electron' },
],
},
{
label: 'Guides',
items: [
{ label: 'Backend Power', slug: 'backend-power' },
{ label: 'Plugins', slug: 'plugins' },
{ label: 'Security', slug: 'security' },
],
},
{
label: 'Frontend Connectors',
autogenerate: { directory: 'frontend' },
},
{
label: 'Reference',
items: [
{ label: 'API Reference', slug: 'api-reference' },
],
}
],
}),
],
});
