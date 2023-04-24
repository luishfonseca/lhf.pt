import * as dotenv from 'dotenv';
import { drizzle } from 'drizzle-orm/planetscale-serverless';
import { migrate } from 'drizzle-orm/planetscale-serverless/migrator';
import { connect } from '@planetscale/database';

dotenv.config();

// create the connection
const connection = connect({
	host: process.env.DATABASE_HOST,
	username: process.env.DATABASE_USERNAME,
	password: process.env.DATABASE_PASSWORD
});

const db = drizzle(connection, { logger: true });

// this will automatically run needed migrations on the database
await migrate(db, { migrationsFolder: './migrations' });
