Esta é a aplicação cliente que irá conectar na API e mostrar os dados no formato de gráficos

# Como testar

```js
cd app/dash/
npm install
npm run dev
```

A aplicação estará disponível na porta 3000

# Obtendo os dados

fazer uma requisicao na api acessando os indicadores

GET http://localhost/api/metricas/ALFA/30
GET http://localhost/api/metricas/GAMA/30

```js
let alfa = [{"label":"1", "value", 39.0},{"label":"2", "value", 39.0},...,{"label":"30", "value", 37.0}]
let beta = [{"label":"1", "value", 199.0},{"label":"2", "value", 179.0},...,{"label":"30", "value", 201.0}]
```
🚧 _em manutenção_

---

# Como foi desenvolvido

Esta aplicação foi desenvolvida com o framework Java Script **svelte** + svelte-tiny-linked-charts lib

```js
npm install --save svelte-tiny-linked-charts

//app.svelte
<script>
import { LinkedChart, LinkedLabel, LinkedValue } from "svelte-tiny-linked-charts"
</script>

<div class="wrapper">
  <LinkedChart { data } />
</div>
```

# Informações detalhadas

Este projeto foi acelerado com vite
```js
npm init vite@latest
√ Project name: ... quiu-dash
√ Select a framework: » svelte
√ Select a variant: » svelte

Scaffolding project in D:\Rust\rust-svelte\quiu-dash...

Done. Now run:

  cd quiu-dash
  npm install
  npm run dev
```
## Technical considerations

**Why use this over SvelteKit?**

- It brings its own routing solution which might not be preferable for some users.
- It is first and foremost a framework that just happens to use Vite under the hood, not a Vite app.
  `vite dev` and `vite build` wouldn't work in a SvelteKit environment, for example.

This template contains as little as possible to get started with Vite + Svelte, while taking into account the developer experience with regards to HMR and intellisense. It demonstrates capabilities on par with the other `create-vite` templates and is a good starting point for beginners dipping their toes into a Vite + Svelte project.

Should you later need the extended capabilities and extensibility provided by SvelteKit, the template has been structured similarly to SvelteKit so that it is easy to migrate.

**Why `global.d.ts` instead of `compilerOptions.types` inside `jsconfig.json` or `tsconfig.json`?**

Setting `compilerOptions.types` shuts out all other types not explicitly listed in the configuration. Using triple-slash references keeps the default TypeScript setting of accepting type information from the entire workspace, while also adding `svelte` and `vite/client` type information.

**Why include `.vscode/extensions.json`?**

Other templates indirectly recommend extensions via the README, but this file allows VS Code to prompt the user to install the recommended extension upon opening the project.

**Why enable `checkJs` in the JS template?**

It is likely that most cases of changing variable types in runtime are likely to be accidental, rather than deliberate. This provides advanced typechecking out of the box. Should you like to take advantage of the dynamically-typed nature of JavaScript, it is trivial to change the configuration.

**Why is HMR not preserving my local component state?**

HMR state preservation comes with a number of gotchas! It has been disabled by default in both `svelte-hmr` and `@sveltejs/vite-plugin-svelte` due to its often surprising behavior. You can read the details [here](https://github.com/rixo/svelte-hmr#svelte-hmr).

If you have state that's important to retain within a component, consider creating an external store which would not be replaced by HMR.

```js
// store.js
// An extremely simple external store
import { writable } from 'svelte/store'
export default writable(0)
```
