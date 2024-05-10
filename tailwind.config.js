import flatten from 'tailwindcss/lib/util/flattenColorPalette';

const config = {
	content: ['./src/**/*.{js,ts,jsx,tsx,mdx}'],
	darkMode: 'class',
	theme: { extend: { boxShadow: { input: '0px 2px 3px -1px rgba(0,0,0,0.1), 0px 1px 0px 0px rgba(25,28,33,0.02), 0px 0px 0px 1px rgba(25,28,33,0.08)' } } },
	plugins: [addVariablesForColors],
};

// This plugin adds each Tailwind color as a global CSS variable, e.g. var(--gray-200).
function addVariablesForColors({ addBase, theme }) {
	const allColors = flatten(theme('colors'));
	const newVars = Object.fromEntries(Object.entries(allColors).map(([key, val]) => [`--${key}`, val]));

	addBase({ ':root': newVars });
}

export default config;
