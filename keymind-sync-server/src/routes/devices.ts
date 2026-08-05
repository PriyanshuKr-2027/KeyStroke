import { FastifyInstance } from "fastify";
import { PrismaClient } from "@prisma/client";
import { AuthenticatedRequest } from "../middleware/auth";

const prisma = new PrismaClient();

export async function deviceRoutes(fastify: FastifyInstance) {
  // GET /api/devices
  fastify.get("/api/devices", async (req: AuthenticatedRequest, reply) => {
    const userId = req.user_id!;

    const devices = await prisma.device.findMany({
      where: { user_id: userId },
      orderBy: { last_active: "desc" },
    });

    return reply.send({ devices });
  });

  // DELETE /api/devices/:id
  fastify.delete(
    "/api/devices/:id",
    async (req: AuthenticatedRequest, reply) => {
      const userId = req.user_id!;
      const { id } = req.params as { id: string };

      const existing = await prisma.device.findFirst({
        where: { id, user_id: userId },
      });

      if (!existing) {
        return reply.status(404).send({ error: "Device not found" });
      }

      await prisma.device.delete({
        where: { id },
      });

      return reply.send({ success: true, message: "Device deregistered" });
    }
  );
}
