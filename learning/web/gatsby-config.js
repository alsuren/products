module.exports = {
  pathPrefix: process.env.PATH_PREFIX || "/",
  siteMetadata: {
    title: `Learning Modules for Continuous Professional Development`,
    author: `MHRA`,
    description: `Medicines and Healthcare products Regulatory Agency Continuous Professional Development`,
    siteUrl: `https://cpd.mhra.gov.uk`,
  },
  plugins: [
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        path: `${__dirname}/content/modules`,
        name: `modules`,
      },
    },
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        path: `${__dirname}/content/assets`,
        name: `assets`,
      },
    },
    `gatsby-transformer-json`,
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        name: `modules`,
        path: `${__dirname}/src/modules`,
      },
    },
    `gatsby-transformer-sharp`,
    `gatsby-plugin-sharp`,
    `gatsby-plugin-offline`,
    `gatsby-plugin-react-helmet`,
    {
      resolve: `gatsby-plugin-typography`,
      options: {
        pathToConfigModule: `src/utils/typography`,
      },
    },
    `gatsby-plugin-styled-components`,
    {
      resolve: `gatsby-plugin-mdx`,
      options: {
        extensions: [`.mdx`, `.md`],
        gatsbyRemarkPlugins: [
          {
            resolve: `gatsby-remark-images`,
            options: {
              maxWidth: 590,
              wrapperStyle: () => `margin: 0;`,
            },
          },
          `gatsby-remark-copy-linked-files`,
        ],
      },
    },
    {
      resolve: `gatsby-plugin-csp`,
      options: {
        disableOnDev: false,
        reportOnly: true,
      },
    },
    `gatsby-plugin-sitemap`,
  ],
}
