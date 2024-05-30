import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';
import tsparser from '@typescript-eslint/parser';
import stylistic from '@stylistic/eslint-plugin';
import globals from 'globals';

export default tseslint.config(
	eslint.configs.recommended,
	...tseslint.configs.recommendedTypeChecked,
	...tseslint.configs.stylisticTypeChecked,
	stylistic.configs.customize({
		indent: 'tab',
		braceStyle: '1tbs',
		semi: true,
	}),
	{
		files: ['next.config.mjs'],
		languageOptions: {
			parserOptions: {
				project: true,
				tsconfigRootDir: import.meta.dirname,
				globals: {
					...globals.node,
					...globals.browser,
					process: 'readonly',
				},
			},
		},
		extends: [tseslint.configs.disableTypeChecked],
	},
	{
		files: ['**/*.js', '**/*.mjs'],
		extends: [tseslint.configs.disableTypeChecked],
	},
	{
		files: ['**/*.ts', '**/*.tsx'],
		ignores: ['src/protogen/**/*.ts'],
		languageOptions: {
			parser: tsparser,
			parserOptions: { project: ['./tsconfig.json'] },
		},
		plugins: { '@stylistic': stylistic },
		rules: {
			'@stylistic/quotes': ['warn', 'single'],
			'@stylistic/semi': ['warn', 'always'],
			'@stylistic/array-bracket-newline': ['warn', { multiline: true }],
			'@stylistic/array-bracket-spacing': ['warn', 'never'],
			'@stylistic/array-element-newline': ['warn', { multiline: true, minItems: 3 }],
			'@stylistic/arrow-parens': 'warn',
			'@stylistic/arrow-spacing': 'warn',
			'@stylistic/block-spacing': 'warn',
			'@stylistic/brace-style': 'warn',
			'@stylistic/comma-dangle': 'warn',
			'@stylistic/comma-spacing': 'warn',
			'@stylistic/comma-style': 'warn',
			'@stylistic/computed-property-spacing': 'warn',
			'@stylistic/dot-location': 'warn',
			'@stylistic/eol-last': 'warn',
			'@stylistic/function-call-argument-newline': ['warn', 'consistent'],
			'@stylistic/function-paren-newline': 'warn',
			'@stylistic/indent': [
				'warn',
				'tab',
				{ SwitchCase: 1 },
			],
			'@stylistic/indent-binary-ops': 'warn',
			'@stylistic/implicit-arrow-linebreak': 'warn',
			'@stylistic/key-spacing': 'warn',
			'@stylistic/keyword-spacing': 'warn',
			'@stylistic/linebreak-style': ['error', 'unix'],
			'@stylistic/lines-between-class-members': 'warn',
			'@stylistic/max-len': 'off',
			'@stylistic/max-statements-per-line': 'warn',
			'@stylistic/member-delimiter-style': 'warn',
			'@stylistic/multiline-ternary': 'warn',
			'@stylistic/new-parens': 'warn',
			'@stylistic/newline-per-chained-call': ['warn', { ignoreChainWithDepth: 3 }],
			'@stylistic/no-extra-parens': 'warn',
			'@stylistic/no-floating-decimal': 'off',
			'@stylistic/no-mixed-operators': 'warn',
			'@stylistic/no-mixed-spaces-and-tabs': 'warn',
			'@stylistic/no-multi-spaces': 'warn',
			'@stylistic/no-trailing-spaces': 'warn',
			'@stylistic/no-whitespace-before-property': 'warn',
			'@stylistic/nonblock-statement-body-position': 'warn',
			'@stylistic/object-curly-newline': [
				'warn',
				{
					ObjectPattern: { multiline: true, minProperties: 3 },
					ImportDeclaration: { multiline: true },
					ExportDeclaration: { multiline: true, minProperties: 3 },
				},
			],
			'@stylistic/object-curly-spacing': ['warn', 'always'],
			'@stylistic/object-property-newline': ['warn', { allowAllPropertiesOnSameLine: true }],
			'@stylistic/operator-linebreak': ['warn', 'before'],
			'@stylistic/padded-blocks': ['warn', 'never'],
			'@stylistic/quote-props': ['warn', 'as-needed'],
			'@stylistic/rest-spread-spacing': 'warn',
			'@stylistic/semi-spacing': 'warn',
			'@stylistic/semi-style': 'warn',
			'@stylistic/space-before-blocks': 'warn',
			'@stylistic/space-before-function-paren': [
				'warn',
				{
					anonymous: 'never',
					named: 'never',
					asyncArrow: 'always',
				},
			],
			'@stylistic/space-in-parens': 'warn',
			'@stylistic/space-infix-ops': 'warn',
			'@stylistic/space-unary-ops': 'warn',
			'@stylistic/spaced-comment': 'warn',
			'@stylistic/template-curly-spacing': 'warn',
			'@stylistic/template-tag-spacing': 'warn',
			'@stylistic/type-annotation-spacing': 'warn',
			'@stylistic/type-generic-spacing': 'warn',
			'@stylistic/type-named-tuple-spacing': 'warn',
			'@stylistic/wrap-iife': 'warn',
			'@stylistic/yield-star-spacing': 'warn',

			'@stylistic/jsx-closing-bracket-location': 'warn',
			'@stylistic/jsx-closing-tag-location': 'warn',
			'@stylistic/jsx-curly-newline': ['warn', { singleline: 'forbid', multiline: 'consistent' }],
			'@stylistic/jsx-curly-spacing': ['warn', { when: 'always', spacing: { objectLiterals: 'never' } }],
			'@stylistic/jsx-equals-spacing': ['warn', 'never'],
			'@stylistic/jsx-first-prop-new-line': ['warn', 'multiline'],
			'@stylistic/jsx-function-call-newline': ['warn', 'multiline'],
			'@stylistic/jsx-indent': [
				'warn',
				'tab',
				{ indentLogicalExpressions: true },
			],
			'@stylistic/jsx-indent-props': ['warn', 'tab'],
			'@stylistic/jsx-max-props-per-line': ['warn', { maximum: 1, when: 'multiline' }],
			'@stylistic/jsx-one-expression-per-line': ['warn', { allow: 'single-child' }],
			'@stylistic/jsx-pascal-case': 'warn',
			'@stylistic/jsx-props-no-multi-spaces': 'warn',
			'@stylistic/jsx-quotes': ['warn', 'prefer-single'],
			'@stylistic/jsx-self-closing-comp': ['warn', { component: true, html: true }],
			'@stylistic/jsx-tag-spacing': ['warn', { beforeClosing: 'never' }],
			'@stylistic/jsx-wrap-multilines': [
				'warn',
				{
					declaration: 'parens-new-line',
					assignment: 'parens-new-line',
					return: 'parens-new-line',
					arrow: 'parens-new-line',
					condition: 'parens-new-line',
					logical: 'parens-new-line',
					prop: 'parens-new-line',
					propertyValue: 'parens-new-line',
				},
			],

			'@typescript-eslint/no-explicit-any': 'off',
			'@typescript-eslint/no-unsafe-assignment': 'off',
			'@typescript-eslint/no-unsafe-call': 'off',
			'@typescript-eslint/no-unsafe-member-access': 'off',
			'@typescript-eslint/no-unsafe-argument': 'off',
			'@typescript-eslint/restrict-plus-operands': 'off',
			'@typescript-eslint/restrict-template-expressions': 'off',
			'@typescript-eslint/no-empty-function': 'off',
			'@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_', varsIgnorePattern: '^_' }],
			'@typescript-eslint/no-misused-promises': 'off',
			'@typescript-eslint/no-redundant-type-constituents': 'off',
		},
	},
);
