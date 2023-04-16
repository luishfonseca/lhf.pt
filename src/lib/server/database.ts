import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export const getPost = async (slug: string) => {
	return await prisma.post.findUnique({
		where: { slug }
	});
};

export const getPosts = async (includeDrafts: boolean) => {
	return await prisma.post.findMany({
		where: { published: includeDrafts ? undefined : true },
		orderBy: { createdAt: 'desc' }
	});
};
