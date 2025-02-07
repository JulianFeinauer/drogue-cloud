# Release cheat sheet

## Next version

Preparing for a new version (not release, like a milestone):

* Change the version in all crates to e.g. `0.4.0`
  * Pay attention to the `service-api` crate as its version will be reported externally

## Overall process

* Get rid of as many as possible "needs release" patches in `Cargo.toml` and `console-frontend/Cargo.toml`
* Create a new tag
  * Start with a `v0.x.0-rc1` version
  * The final version should be `v0.x.0`
* Push the tag
* Wait for the build
* Test the instructions in the following "Installation" subsections
* For each installation:
  * Test the links on the command line
  * Test the links in the web console
  * Try out the example commands
* Create a branch `release-0.x`
  * Ensure to switch the doc version to 0.x too: `docs/antora.yml`

## Release text

The text that goes into the final GitHub release record comes from `installer/README.md`
