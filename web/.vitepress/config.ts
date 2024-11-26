import { defineConfig } from 'vitepress'
import { tabsMarkdownPlugin } from 'vitepress-plugin-tabs'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Lead",
  description: "An Experimental Programming Language",
  appearance: "dark",
  base: "/lead/",
  lang: "en-US",
  head: [
    ["link", { rel: "icon", href: "/lead/icon.svg" }]
  ],
  themeConfig: {
    search: {
      provider: "local"
    },
    logo: "/icon.svg",
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Download', link: '/download' },
      { text: 'Examples', link: '/markdown-examples' }
    ],

    sidebar: [
      {
        text: 'Download',
        items: [
          { text: "Get Lead Lang", link: "/download" }
        ]
      },
      {
        text: 'Examples',
        items: [

        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vuejs/vitepress' }
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: `Copyright © 2023-${new Date().getFullYear()} Lead Programming Language`
    }
  },
  markdown: {
    config(md) {
      md.use(tabsMarkdownPlugin)
    }
  }
})