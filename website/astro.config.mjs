// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	integrations: [
		starlight({
			title: 'Hydent PL',
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/sonneko/hydent-lang' }],
			sidebar: [
				{
					label: 'Guides',
					autogenerate: { directory: 'docs' },
				},
				{
					label: 'Reference',
					autogenerate: { directory: 'reference' },
				},
			],
		}),
	],
});
