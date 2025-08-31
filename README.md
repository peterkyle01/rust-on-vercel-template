# 🦀 Rust on Vercel Template - Production Ready

A complete full-stack application template combining **Rust serverless functions** with **Next.js frontend**, featuring JWT authentication, PostgreSQL database integration, and seamless Vercel deployment.

## ✨ Features

- 🔐 **Complete JWT Authentication System** with secure password hashing
- 🗄️ **PostgreSQL Database Integration** with SQLx and migrations
- 🚀 **Vercel Deployment Ready** with offline compilation support
- ⚡ **Next.js 15** with Turbopack for lightning-fast development
- 🎨 **Beautiful UI** with Tailwind CSS and shadcn/ui components
- 🔒 **Enterprise-Grade Security** with bcrypt and JWT
- 📱 **Responsive Design** that works on all devices
- 🔄 **Type Safety** with auto-generated TypeScript types from Rust structs

## 🚀 Quick Start

### 1. Environment Setup

```bash
# Clone and setup
git clone <your-repo>
cd rust-on-vercel-template

# Copy environment file
cp .env.example .env
```

### 2. Configure Environment Variables

Update your `.env` file:

```bash
# Database (get free PostgreSQL from neon.tech)
DATABASE_URL=postgresql://username:password@your-neon-db.com/database

# JWT Secret (generate a secure random string)
JWT_SECRET=your-super-secure-jwt-secret-here

# Build Configuration (required for offline compilation)
SQLX_OFFLINE=true
```

### 3. Install Dependencies

```bash
npm install
```

### 4. Setup Database (Optional - works offline too!)

```bash
# If you have a database connection
npm run db:setup

# Or work completely offline
npm run generate:types
```

### 5. Start Development

```bash
vercel dev
```

Visit [http://localhost:3000](http://localhost:3000) to see your application!

## 📋 Available Commands

| Command                      | Description                                     |
| ---------------------------- | ----------------------------------------------- |
| `npm run dev`                | Start Next.js development server with Turbopack |
| `npm run build`              | Build for production                            |
| `npm run start`              | Start production server                         |
| `npm run lint`               | Run ESLint                                      |
| `npm run generate:types`     | Generate TypeScript types from Rust structs     |
| `npm run db:migrate`         | Run database migrations                         |
| `npm run db:setup`           | Setup database and generate types               |
| `npm run db:prepare`         | Prepare SQLx queries for offline compilation    |
| `npm run rust:build`         | Build Rust code in release mode                 |
| `npm run rust:check`         | Check Rust code for errors                      |
| `npm run rust:build-offline` | Build Rust code with forced offline mode        |
| `npm run rust:check-offline` | Check Rust code with forced offline mode        |

## 🏗️ Architecture

### Frontend Stack

- **Next.js 15** with App Router
- **React 19** with modern hooks
- **TypeScript** for type safety
- **Tailwind CSS** for styling
- **shadcn/ui** for beautiful components

### Backend Stack

- **Rust** with Vercel Runtime
- **SQLx** for database operations
- **PostgreSQL** with Neon hosting
- **JWT** for authentication
- **bcrypt** for password hashing

## 📡 API Endpoints

### Authentication

#### POST /api/auth/signup

Create a new user account.

**Request:**

```json
{
  "email": "user@example.com",
  "username": "johndoe",
  "password": "securepassword123"
}
```

**Response:**

```json
{
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "johndoe",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  },
  "token": "jwt_token_here"
}
```

#### POST /api/auth/signin

Sign in with existing credentials.

**Request:**

```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

#### GET /api/auth/me

Get current user information (requires authentication).

**Headers:**

```
Authorization: Bearer your_jwt_token_here
```

### Protected Routes

#### GET /api/routes/products

Get list of products (requires authentication).

**Headers:**

```
Authorization: Bearer your_jwt_token_here
```

**Response:**

```json
[
  {
    "id": "1",
    "name": "Laptop",
    "price": 999.99
  },
  {
    "id": "2",
    "name": "Mouse",
    "price": 29.99
  }
]
```

## 🔒 Security Features

- ✅ **Password Hashing** with bcrypt and configurable cost
- ✅ **JWT Authentication** with secure token generation
- ✅ **Input Validation** and sanitization
- ✅ **Secure Error Handling** without sensitive data leakage
- ✅ **Environment Variable Protection**
- ✅ **SQL Injection Prevention** with parameterized queries

## 🎯 Offline Compilation Mode

This project supports **SQLx offline mode**, allowing you to build and compile without requiring an active database connection. Perfect for:

- **CI/CD pipelines** where database access might not be available
- **Local development** when you want to work on frontend code
- **Vercel deployments** where the build process runs separately from runtime

### How it works:

Set `SQLX_OFFLINE=true` in your environment:

```bash
# In your .env file
SQLX_OFFLINE=true

# Then build normally
npm run rust:check
npm run generate:types
```

## 🚀 Deploy to Vercel

### Environment Variables Setup

In your Vercel dashboard, set these environment variables:

```bash
# Build Configuration (CRITICAL for Vercel builds)
SQLX_OFFLINE=true

# Runtime Configuration
DATABASE_URL=postgresql://username:password@your-neon-db.com/database
JWT_SECRET=your-super-secure-jwt-secret-here
```

### Deployment Steps

1. **Push to GitHub**: Make sure your code is pushed to GitHub
2. **Connect to Vercel**: Import your repository in Vercel dashboard
3. **Set Environment Variables**: Add the required variables in Vercel → Project Settings → Environment Variables
4. **Deploy**: Vercel will automatically build and deploy

### Why SQLX_OFFLINE=true is Required

Vercel's build environment doesn't have access to your database during the build process. Setting `SQLX_OFFLINE=true` tells SQLx to use pre-generated query metadata instead of trying to connect to the database during compilation.

## 🛠️ Development

### Database Setup with Neon

1. Create a free PostgreSQL database at [neon.tech](https://neon.tech)
2. Update your `.env` file with the connection string
3. Run migrations:

```bash
npm run db:migrate
```

### Adding New API Endpoints

1. Create a new Rust file in the `api/` directory
2. Add the binary target to `Cargo.toml`
3. Follow the existing pattern for authentication and error handling

### Database Migrations

Create new migration files in the `migrations/` directory:

```sql
-- migrations/002_add_products_table.sql
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    price DECIMAL(10,2) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

Then run:

```bash
npm run db:migrate
```

### Generating TypeScript Types

After modifying Rust structs with `#[derive(TS)]`, regenerate TypeScript types:

```bash
npm run generate:types
```

This updates `types/models.ts` with the latest TypeScript interfaces.

## 📦 Dependencies & Acknowledgments

### 🎉 Special Thanks to Amazing Open Source Projects

#### Frontend Dependencies

- **[Next.js](https://nextjs.org/)** by Vercel - The React framework that makes web development a joy
- **[React](https://reactjs.org/)** by Meta - The library that revolutionized UI development
- **[Tailwind CSS](https://tailwindcss.com/)** by Tailwind Labs - Utility-first CSS framework that's a game changer
- **[Radix UI](https://www.radix-ui.com/)** - Unstyled, accessible components for building design systems
- **[shadcn/ui](https://ui.shadcn.com/)** - Beautifully designed components built on Radix UI and Tailwind CSS
- **[Lucide React](https://lucide.dev/)** - Beautiful & consistent icon toolkit

#### Rust Crates

- **[sqlx](https://github.com/launchbadge/sqlx)** by launchbadge - Amazing async SQL toolkit for Rust
- **[serde](https://serde.rs/)** by dtolnay - The gold standard for serialization in Rust
- **[tokio](https://tokio.rs/)** - Asynchronous runtime for Rust that powers millions of applications
- **[vercel_runtime](https://github.com/vercel/vercel)** by Vercel - Seamless Rust integration with Vercel
- **[jsonwebtoken](https://github.com/Keats/jsonwebtoken)** by Keats - Robust JWT implementation for Rust
- **[bcrypt](https://github.com/Keats/rust-bcrypt)** by Keats - Secure password hashing
- **[uuid](https://github.com/uuid-rs/uuid)** - RFC 4122 UUID implementation
- **[chrono](https://github.com/chronotope/chrono)** - Date and time library for Rust
- **[anyhow](https://github.com/dtolnay/anyhow)** by dtolnay - Flexible error handling
- **[ts-rs](https://github.com/Aleph-Alpha/ts-rs)** by Aleph-Alpha - Generate TypeScript types from Rust

#### Development Tools

- **[TypeScript](https://www.typescriptlang.org/)** by Microsoft - Static type checking for JavaScript
- **[ESLint](https://eslint.org/)** - Pluggable linting utility for JavaScript and TypeScript
- **[PostCSS](https://postcss.org/)** - Tool for transforming CSS with JavaScript

### 💝 Community Appreciation

A huge thank you to the entire **Rust community** for building such an incredible ecosystem. The combination of safety, performance, and developer experience is unmatched.

Special appreciation to the **Next.js team** at Vercel for continuously pushing the boundaries of web development and making deployment seamless.

Gratitude to all the maintainers and contributors of the open source projects that make this template possible. Your dedication to creating high-quality, free software benefits developers worldwide. 🙏

## 📄 Error Responses

All endpoints return errors in a consistent format:

```json
{
  "message": "Error description",
  "code": 400
}
```

**Common Error Codes:**

- `400` - Bad Request (validation errors)
- `401` - Unauthorized (missing/invalid token)
- `404` - Not Found
- `405` - Method Not Allowed
- `500` - Internal Server Error

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

This project is open source and available under the [MIT License](LICENSE).

---

**Built with ❤️ using Rust and Next.js**
