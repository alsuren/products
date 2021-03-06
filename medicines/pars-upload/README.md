This is the PARS upload form.

- See an [example use of the form][example doc].

## Running locally

First, you'll need `yarn` installed.

Once you have `yarn` installed, run:

```sh
yarn && yarn dev
```

This installs the package and runs the website at [localhost:3000](http://localhost:3000).

To understand how to use it, see the [example use of the form][example doc].

[example doc]: ./docs/example.md


## Running acceptance tests

Browser based acceptance tests are run using [cypress](https://www.cypress.io).

```sh
yarn test-e2e
```

## Authentication

The pages are protected by Azure Active Directory Single Sign On (SSO).
 
The `Sign In` button will display a popup window where Microsoft Identity Authentication SSO will present itself.
Users will authenticate in the normal way for their Azure Active Directory, including 2FA if it is setup on the account.

`NEXT_PUBLIC_AUTHORITY_URL` and `NEXT_PUBLIC_CLIENT_ID` are set in `.env` and used to identify the [Azure App Registration](https://docs.microsoft.com/en-us/azure/active-directory/develop/msal-client-application-configuration) to use.

The user is authenticated against the configured Azure App Registration and a JWT token is stored on their behalf in browser session storage.

Use the following command to populate `.env` from Azure Key Vault.

```sh
make get-env
```

## Browser requirements

This site is rendered client-side using _React_. Users must have JavaScript enabled.

We support IE11 browsers and later, including all versions of Edge, Firefox 21+ and Chrome 23+. This aligns with [ECMAScript 5][caniuse ES5].

[caniuse ES5]: https://caniuse.com/#feat=es5
