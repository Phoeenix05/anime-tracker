import devtools from 'solid-devtools/vite'
import { defineConfig } from 'vite'
import solidPlugin from 'vite-plugin-solid'

export default defineConfig({
	plugins: [solidPlugin(), devtools({ autoname: true })],
	server: {
		port: 8000,
	},
	build: {
		target: 'esnext',
	},
})
