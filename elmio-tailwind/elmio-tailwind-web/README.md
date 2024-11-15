# node-ts-template

A minimal template for bootstrapping Node.js and TypeScript projects.

This template includes pre-configured tools to ensure consistent code quality, style, and testing practices.

## Features

- **Biome**: For formatting, linting, and organizing imports.  
  Learn more: [biomejs.dev](https://biomejs.dev/guides/getting-started/)
- **Vitest**: For testing.  
  Learn more: [vitest.dev](https://vitest.dev/guide/)

## Usage

### 1. Clone the Repository

```bash
git clone https://github.com/xosnrdev/node-ts-template.git your-project-name
cd your-project-name
```

### 2. Install Dependencies

```bash
pnpm install
```

### 3. Start Developing

Replace the placeholder files with your project code.

Use the following commands to maintain code quality and run tests:

```bash
pnpm check       # Check code for issues with Biome
pnpm fmt         # Format code with Biome
pnpm test        # Run tests with Vitest
pnpm fix         # Automatically fix issues with Biome
pnpm build      # Compile TypeScript with tsc
```

## License

This project is licensed under the [MIT License](./LICENSE).
