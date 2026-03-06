// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://sonneko.github.io',
	base: '/hydent-lang',
	integrations: [
		starlight({
			title: 'Hydent PL',
			customCss: [
				'./src/styles/index.css',
			],
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
				{
					label: 'ForDevelopers',
					autogenerate: { directory: 'developers' },
				}
			],
		}),
	],
});
