# Tailwind CSS benchmark fixture generator

This project uses Rspack and Tailwind CSS to regenerate
`../../files/tailwind.css`.

The generator asks Tailwind's pinned design system for every finite utility in
the default theme. It also adds one example for every supported modifier on
each utility family and one example for every finite default variant. Arbitrary
values and arbitrary variants are intentionally excluded because their input
space is infinite.

The Tailwind PostCSS optimizer and Rspack minimizer are both disabled so the
fixture remains unminified.

```sh
pnpm install --frozen-lockfile
pnpm generate
```
