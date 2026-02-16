# EtchDNS 🌐

![EtchDNS](https://img.shields.io/badge/EtchDNS-v1.0.0-blue.svg)  
[![GitHub Releases](https://img.shields.io/badge/releases-latest-orange.svg)](https://github.com/aldebaran1805/EtchDNS/releases)

Welcome to **EtchDNS**, a new DNS proxy designed for simplicity, security, and extensibility with WebAssembly plugins. This repository aims to provide an efficient and secure way to manage DNS queries while offering flexibility through its plugin architecture.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Plugins](#plugins)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Features

- **Simplicity**: Easy to set up and use.
- **Security**: Built with security in mind to protect against common threats.
- **Extensibility**: Supports WebAssembly plugins for custom functionality.
- **Caching**: Efficient DNS caching to speed up queries.
- **Compatibility**: Works seamlessly with existing DNS infrastructure.

## Installation

To get started with EtchDNS, you can download the latest release from the [Releases section](https://github.com/aldebaran1805/EtchDNS/releases). After downloading, follow these steps to install:

1. Extract the downloaded file.
2. Move the executable to a directory in your PATH.
3. Run the installation script (if provided).

## Usage

Once installed, you can start using EtchDNS with the following command:

```bash
etchdns start
```

This command will initiate the DNS proxy, and you can begin routing your DNS queries through it. 

To check the status of the service, use:

```bash
etchdns status
```

For stopping the service, simply run:

```bash
etchdns stop
```

## Configuration

EtchDNS uses a configuration file located at `/etc/etchdns/config.yaml`. Here’s a basic example of what the configuration might look like:

```yaml
server:
  address: "0.0.0.0"
  port: 53

cache:
  enabled: true
  ttl: 3600

plugins:
  - name: "example-plugin"
    path: "/path/to/plugin.wasm"
```

Make sure to adjust the parameters according to your needs. 

## Plugins

EtchDNS allows you to extend its functionality through WebAssembly plugins. You can create your own plugins or use existing ones. Here’s how to load a plugin:

1. Write your WebAssembly plugin and compile it.
2. Add the plugin path to the configuration file under the `plugins` section.
3. Restart the EtchDNS service to load the new plugin.

For more information on writing WebAssembly plugins, check the [WebAssembly documentation](https://webassembly.org/docs/).

## Contributing

We welcome contributions to EtchDNS. If you want to help improve the project, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them with clear messages.
4. Push your branch to your fork.
5. Open a pull request.

Please ensure that your code adheres to our coding standards and includes appropriate tests.

## License

EtchDNS is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contact

For any inquiries or issues, feel free to reach out:

- **GitHub**: [aldebaran1805](https://github.com/aldebaran1805)
- **Email**: your-email@example.com

Thank you for checking out EtchDNS! For the latest releases, visit the [Releases section](https://github.com/aldebaran1805/EtchDNS/releases) to download and execute the latest version.