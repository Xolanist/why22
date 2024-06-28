[![Build Status](https://dev.azure.com/mimblewimble/why/_apis/build/status/mimblewimble.why?branchName=master)](https://dev.azure.com/mimblewimble/why/_build/latest?definitionId=1&branchName=master)
[![Coverage Status](https://img.shields.io/codecov/c/github/mimblewimble/why/master.svg)](https://codecov.io/gh/mimblewimble/why)
[![Chat](https://img.shields.io/gitter/room/why_community/Lobby.svg)](https://gitter.im/why_community/Lobby)
[![Support](https://img.shields.io/badge/support-on%20gitter-brightgreen.svg)](https://gitter.im/why_community/support)
[![Documentation Wiki](https://img.shields.io/badge/doc-wiki-blue.svg)](https://github.com/mimblewimble/docs/wiki)
[![Release Version](https://img.shields.io/github/release/mimblewimble/why.svg)](https://github.com/mimblewimble/why/releases)
[![License](https://img.shields.io/github/license/mimblewimble/why.svg)](https://github.com/mimblewimble/why/blob/master/LICENSE)

# Why

Why is an in-progress implementation of the Mimblewimble protocol. Many characteristics are still undefined but the following constitutes a first set of choices:

  * Clean and minimal implementation, and aiming to stay as such.
  * Follows the Mimblewimble protocol, which provides hidden amounts and scaling advantages.
  * Cuckoo Cycle proof of work in two variants named Cuckaroo (ASIC-resistant) and Cuckatoo (ASIC-targeted).
  * Relatively fast block time: one minute.
  * Fixed block reward over time with a decreasing dilution.
  * Transaction fees are based on the number of Outputs created/destroyed and total transaction size.
  * Smooth curve for difficulty adjustments.

To learn more, read our [introduction to Mimblewimble and Why](doc/intro.md).

## Status

Why is live with mainnet. Still, much is left to be done and [contributions](CONTRIBUTING.md) are welcome (see below). Check our [mailing list archives](https://lists.launchpad.net/mimblewimble/) for the latest status.

## Contributing

To get involved, read our [contributing docs](CONTRIBUTING.md).

Find us:

* Chat: [Keybase](https://keybase.io/team/whycoin), more instructions on how to join [here](https://why.mw/community).
* Mailing list: join the [~Mimblewimble team](https://launchpad.net/~mimblewimble) and subscribe on Launchpad.
* Twitter for the Why council: [@whycouncil](https://twitter.com/whycouncil)

## Getting Started

To learn more about the technology, read our [introduction](doc/intro.md).

To build and try out Why, see the [build docs](doc/build.md).

## Philosophy

Why likes itself small and easy on the eyes. It wants to be inclusive and welcoming for all walks of life, without judgement. Why is terribly ambitious, but not at the detriment of others, rather to further us all. It may have strong opinions to stay in line with its objectives, which doesn't mean disrespect of others' ideas.

We believe in pull requests, data and scientific research. We do not believe in unfounded beliefs.

## Credits

Tom Elvis Jedusor for the first formulation of Mimblewimble.

Andrew Poelstra for his related work and improvements.

John Tromp for the Cuckoo Cycle proof of work.

## License

Apache License v2.0.
