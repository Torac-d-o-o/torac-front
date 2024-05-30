import { join } from 'path'
import { skeleton } from '@skeletonlabs/tw-plugin'
import forms from '@tailwindcss/forms'

/** @type {import('tailwindcss').Config} */
export default {
    darkMode: 'class',
    content: [
        './src/**/*.{html,js,svelte,ts}',
        join(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,js,svelte,ts}')
    ],
    theme: {
        extend: {}
    },
    plugins: [
        forms,
        skeleton({
            themes: {
                custom: [
                    {
                        name: 'torac',
                        properties: {
                            // =~= Theme Properties =~=
                            '--theme-font-family-base':
                                'Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, \'Segoe UI\', Roboto, \'Helvetica Neue\', Arial, \'Noto Sans\', sans-serif, \'Apple Color Emoji\', \'Segoe UI Emoji\', \'Segoe UI Symbol\', \'Noto Color Emoji\'',
                            '--theme-font-family-heading':
                                'Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, \'Segoe UI\', Roboto, \'Helvetica Neue\', Arial, \'Noto Sans\', sans-serif, \'Apple Color Emoji\', \'Segoe UI Emoji\', \'Segoe UI Symbol\', \'Noto Color Emoji\'',
                            '--theme-font-color-base': '0 0 0',
                            '--theme-font-color-dark': '255 255 255',
                            '--theme-rounded-base': '9999px',
                            '--theme-rounded-container': '8px',
                            '--theme-border-base': '1px',
                            // =~= Theme On-X Colors =~=
                            '--on-primary': '0 0 0',
                            '--on-secondary': '255 255 255',
                            '--on-tertiary': '0 0 0',
                            '--on-success': '0 0 0',
                            '--on-warning': '0 0 0',
                            '--on-error': '255 255 255',
                            '--on-surface': '255 255 255',
                            // =~= Theme Colors  =~=
                            // primary | #71847b
                            '--color-primary-50': '234 237 235', // #eaedeb
                            '--color-primary-100': '227 230 229', // #e3e6e5
                            '--color-primary-200': '220 224 222', // #dce0de
                            '--color-primary-300': '198 206 202', // #c6ceca
                            '--color-primary-400': '156 169 163', // #9ca9a3
                            '--color-primary-500': '113 132 123', // #71847b
                            '--color-primary-600': '102 119 111', // #66776f
                            '--color-primary-700': '85 99 92', // #55635c
                            '--color-primary-800': '68 79 74', // #444f4a
                            '--color-primary-900': '55 65 60', // #37413c
                            // secondary | #4f4d3a
                            '--color-secondary-50': '229 228 225', // #e5e4e1
                            '--color-secondary-100': '220 219 216', // #dcdbd8
                            '--color-secondary-200': '211 211 206', // #d3d3ce
                            '--color-secondary-300': '185 184 176', // #b9b8b0
                            '--color-secondary-400': '132 130 117', // #848275
                            '--color-secondary-500': '79 77 58', // #4f4d3a
                            '--color-secondary-600': '71 69 52', // #474534
                            '--color-secondary-700': '59 58 44', // #3b3a2c
                            '--color-secondary-800': '47 46 35', // #2f2e23
                            '--color-secondary-900': '39 38 28', // #27261c
                            // tertiary | #84717A
                            '--color-tertiary-50': '237 234 235', // #edeaeb
                            '--color-tertiary-100': '230 227 228', // #e6e3e4
                            '--color-tertiary-200': '224 220 222', // #e0dcde
                            '--color-tertiary-300': '206 198 202', // #cec6ca
                            '--color-tertiary-400': '169 156 162', // #a99ca2
                            '--color-tertiary-500': '132 113 122', // #84717A
                            '--color-tertiary-600': '119 102 110', // #77666e
                            '--color-tertiary-700': '99 85 92', // #63555c
                            '--color-tertiary-800': '79 68 73', // #4f4449
                            '--color-tertiary-900': '65 55 60', // #41373c
                            // success | #84cc16
                            '--color-success-50': '237 247 220', // #edf7dc
                            '--color-success-100': '230 245 208', // #e6f5d0
                            '--color-success-200': '224 242 197', // #e0f2c5
                            '--color-success-300': '206 235 162', // #ceeba2
                            '--color-success-400': '169 219 92', // #a9db5c
                            '--color-success-500': '132 204 22', // #84cc16
                            '--color-success-600': '119 184 20', // #77b814
                            '--color-success-700': '99 153 17', // #639911
                            '--color-success-800': '79 122 13', // #4f7a0d
                            '--color-success-900': '65 100 11', // #41640b
                            // warning | #EAB308
                            '--color-warning-50': '252 244 218', // #fcf4da
                            '--color-warning-100': '251 240 206', // #fbf0ce
                            '--color-warning-200': '250 236 193', // #faecc1
                            '--color-warning-300': '247 225 156', // #f7e19c
                            '--color-warning-400': '240 202 82', // #f0ca52
                            '--color-warning-500': '234 179 8', // #EAB308
                            '--color-warning-600': '211 161 7', // #d3a107
                            '--color-warning-700': '176 134 6', // #b08606
                            '--color-warning-800': '140 107 5', // #8c6b05
                            '--color-warning-900': '115 88 4', // #735804
                            // error | #a1221b
                            '--color-error-50': '241 222 221', // #f1dedd
                            '--color-error-100': '236 211 209', // #ecd3d1
                            '--color-error-200': '232 200 198', // #e8c8c6
                            '--color-error-300': '217 167 164', // #d9a7a4
                            '--color-error-400': '189 100 95', // #bd645f
                            '--color-error-500': '161 34 27', // #a1221b
                            '--color-error-600': '145 31 24', // #911f18
                            '--color-error-700': '121 26 20', // #791a14
                            '--color-error-800': '97 20 16', // #611410
                            '--color-error-900': '79 17 13', // #4f110d
                            // surface | #2d3431
                            '--color-surface-50': '224 225 224', // #e0e1e0
                            '--color-surface-100': '213 214 214', // #d5d6d6
                            '--color-surface-200': '203 204 204', // #cbcccc
                            '--color-surface-300': '171 174 173', // #abaead
                            '--color-surface-400': '108 113 111', // #6c716f
                            '--color-surface-500': '45 52 49', // #2d3431
                            '--color-surface-600': '41 47 44', // #292f2c
                            '--color-surface-700': '34 39 37', // #222725
                            '--color-surface-800': '27 31 29', // #1b1f1d
                            '--color-surface-900': '22 25 24' // #161918
                        }
                    }
                ]
            }
        })
    ]
}
