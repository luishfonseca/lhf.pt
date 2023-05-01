import { drizzle } from 'drizzle-orm/mysql2';
import { migrate } from 'drizzle-orm/mysql2/migrator';
import mysql from 'mysql2/promise';

if (!process.env.DATABASE_URL) {
	console.log('Missing DATABASE_URL environment variable');
	process.exit(1);
}

const connection = await mysql.createConnection(process.env.DATABASE_URL);

const db = drizzle(connection, { logger: true });

migrate(db, { migrationsFolder: './migrations' }).finally(() => {
	process.exit();
});
