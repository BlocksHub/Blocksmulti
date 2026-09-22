<div align="center">

# 🚀 Blocksmulti

[![Build and Release](https://github.com/BlocksHub/Blockmulti/actions/workflows/build-and-release.yml/badge.svg)](https://github.com/BlocksHub/Blockmulti/actions/workflows/build-and-release.yml)
[![License: CeCILL v2.1](https://img.shields.io/badge/License-CeCILL_v2.1-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
[![Swift](https://img.shields.io/badge/Swift-FA7343?logo=swift&logoColor=white)](https://swift.org)
[![Kotlin](https://img.shields.io/badge/Kotlin-0095D5?logo=kotlin&logoColor=white)](https://kotlinlang.org/)
[![UniFFI](https://img.shields.io/badge/UniFFI-Mozilla-red)](https://mozilla.github.io/uniffi-rs/)

A perfect wrapper for Esup Multi API, written in Rust and automatically generating bindings for **Swift** (iOS) and **Kotlin** (Android) using [UniFFI](https://mozilla.github.io/uniffi-rs/).

</div>

> [!IMPORTANT]
> This package is not affiliated with any institutions and is not officially supported by any institutions. We are not responsible for any misuse of this package. This package is intended to help students and staff to interact with the internal API of Esup Multi.

## ✨ Supported Features

This library supports various backend features for Esup Multi:

- 🔐 **Authentication**: Direct Login, Logout, CAS / SSO support
- 🎓 **User Cards**: Standard and European Student Card formats
- 📅 **Schedule**: Timetable viewing
- 🔔 **Notifications**: Fetch, mark as read, Push Notifications via FCM
- ⏱️ **Clocking**: Staff clock-in/out support
- 🍔 **Restaurants**: List CROUS restaurants and their menus
- 📰 **RSS & News**: Important news and RSS feeds
- 📱 **Social Networks**: Links to university social media
- 📄 **Static Pages**: Informational university pages
- 📧 **Mail & Calendar**: Mail and Calendar access
- 🗺️ **Map**: Interactive campus maps and points of interest
- 📞 **Contacts**: Directory search and contact forms
- 📊 **Statistics**: User action statistics tracking

## 🏫 Supported Universities (Endpoints)

Here is a list of supported universities with their respective API endpoints and authentication methods:

| University | API Endpoint | Authentication |
| :--- | :--- | :--- |
| 🏛️ **Université Polytechnique Hauts-de-France (UPHF)** | [https://appmob.uphf.fr/backend/](https://appmob.uphf.fr/backend/) | DIRECT |
| 🏛️ **Institut National des Sciences Appliquées de Lyon (INSA Lyon)** | [https://api.moninsalyon.insa-lyon.fr/api/](https://api.moninsalyon.insa-lyon.fr/api/) | CAS |
| 🏛️ **Université Rennes 2** | [https://gateway.r2mobile.univ-rennes2.fr/](https://gateway.r2mobile.univ-rennes2.fr/) | DIRECT |
| 🏛️ **Université de Reims Champagne-Ardenne (URCA)** | [https://mobile-backend.univ-reims.fr](https://mobile-backend.univ-reims.fr) | DIRECT |
| 🏛️ **Université de Nîmes** | [https://mobile-back.unimes.fr](https://mobile-back.unimes.fr) | DIRECT |
| 🏛️ **Université de Lorraine** | [https://mobile-back.univ-lorraine.fr](https://mobile-back.univ-lorraine.fr) | DIRECT |
| 🏛️ **Normandie Université** | [https://enpoche.normandie-univ.fr](https://enpoche.normandie-univ.fr) | CAS |

> [!INFO]
> This list is not exhaustive and may not be up to date. If you know of another university that uses Esup Multi, please let us know and we will add it to the list.
> You can view the full list of supported universities on the [Esup Multi documentation](https://wiki.esup-portail.org/xwiki/bin/view/ProjetMulti/DebuterAvecMulti).

## 📜 License

This project is the property of **BlockHub** and is distributed under the [CeCILL v2.1](LICENSE) license.

## 🙏 Acknowledgments

- [🏛️ Université de Lorraine](https://www.univ-lorraine.fr/) for [esup-multi](https://github.com/univlorraine/esup-multi/).
- [🏛️ UPHF (Université Polytechnique Haut-de-France)](https://www.uphf.fr/) for the account.
- [👨‍🎨 Raphaël SCHRÖDER](https://github.com/raphckrman) for the structure of the library.
