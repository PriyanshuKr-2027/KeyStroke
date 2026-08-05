import { FastifyInstance } from "fastify";
import { PrismaClient } from "@prisma/client";
import { AuthenticatedRequest } from "../middleware/auth";
import { rateLimitExport } from "../middleware/rateLimiter";

const prisma = new PrismaClient();

export async function accountRoutes(fastify: FastifyInstance) {
  // GET /api/account/export (GDPR data dump)
  fastify.get(
    "/api/account/export",
    { preHandler: [rateLimitExport] },
    async (req: AuthenticatedRequest, reply) => {
      const userId = req.user_id!;

      const user = await prisma.user.findUnique({
        where: { id: userId },
        include: {
          devices: true,
          variables: true,
          dictionaryWords: true,
          memoryItems: true,
        },
      });

      if (!user) {
        return reply.status(404).send({ error: "User account not found" });
      }

      return reply.send({
        exported_at: new Date().toISOString(),
        user_data: user,
      });
    }
  );

  // DELETE /api/account (GDPR right to erasure)
  fastify.delete("/api/account", async (req: AuthenticatedRequest, reply) => {
    const userId = req.user_id!;

    // Cascade delete user data
    await prisma.user.delete({
      where: { id: userId },
    });

    return reply.send({
      success: true,
      message: "User account and all associated sync data permanently deleted.",
    });
  });
}
