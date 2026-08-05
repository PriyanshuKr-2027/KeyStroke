import { FastifyInstance } from "fastify";
import { PrismaClient } from "@prisma/client";
import { AuthenticatedRequest } from "../middleware/auth";
import { rateLimitSync } from "../middleware/rateLimiter";

const prisma = new PrismaClient();

interface SyncPushBody {
  device_id: string;
  last_sync_at?: string;
  delta: {
    variables?: Array<{
      id: string;
      key: string;
      var_type: string;
      value?: string;
      ai_prompt?: string;
      use_count?: number;
      deleted_at?: string;
    }>;
    dictionary?: Array<{
      id: string;
      word: string;
      deleted_at?: string;
    }>;
    memory?: Array<{
      id: string;
      phrase: string;
      frequency?: number;
      is_pinned?: boolean;
      deleted_at?: string;
    }>;
  };
}

export async function syncRoutes(fastify: FastifyInstance) {
  // POST /api/sync/push
  fastify.post(
    "/api/sync/push",
    { preHandler: [rateLimitSync] },
    async (req: AuthenticatedRequest, reply) => {
      const userId = req.user_id!;
      const body = req.body as SyncPushBody;
      const now = new Date();

      const { variables, dictionary, memory } = body.delta || {};

      // 1. Ensure user exists
      await prisma.user.upsert({
        where: { id: userId },
        update: { updated_at: now },
        create: { id: userId, email: req.user_email || `${userId}@keymind.app` },
      });

      // 2. Upsert Variables
      if (variables && variables.length > 0) {
        for (const v of variables) {
          await prisma.variable.upsert({
            where: { id: v.id },
            update: {
              key: v.key,
              var_type: v.var_type,
              value: v.value,
              ai_prompt: v.ai_prompt,
              use_count: v.use_count ?? 0,
              updated_at: now,
              deleted_at: v.deleted_at ? new Date(v.deleted_at) : null,
            },
            create: {
              id: v.id,
              user_id: userId,
              key: v.key,
              var_type: v.var_type,
              value: v.value,
              ai_prompt: v.ai_prompt,
              use_count: v.use_count ?? 0,
              updated_at: now,
              deleted_at: v.deleted_at ? new Date(v.deleted_at) : null,
            },
          });
        }
      }

      // 3. Upsert Dictionary Words
      if (dictionary && dictionary.length > 0) {
        for (const dw of dictionary) {
          await prisma.dictionaryWord.upsert({
            where: { id: dw.id },
            update: {
              word: dw.word,
              updated_at: now,
              deleted_at: dw.deleted_at ? new Date(dw.deleted_at) : null,
            },
            create: {
              id: dw.id,
              user_id: userId,
              word: dw.word,
              updated_at: now,
              deleted_at: dw.deleted_at ? new Date(dw.deleted_at) : null,
            },
          });
        }
      }

      // 4. Upsert Memory Items
      if (memory && memory.length > 0) {
        for (const m of memory) {
          await prisma.memoryItem.upsert({
            where: { id: m.id },
            update: {
              phrase: m.phrase,
              frequency: m.frequency ?? 1,
              is_pinned: m.is_pinned ?? false,
              updated_at: now,
              deleted_at: m.deleted_at ? new Date(m.deleted_at) : null,
            },
            create: {
              id: m.id,
              user_id: userId,
              phrase: m.phrase,
              frequency: m.frequency ?? 1,
              is_pinned: m.is_pinned ?? false,
              updated_at: now,
              deleted_at: m.deleted_at ? new Date(m.deleted_at) : null,
            },
          });
        }
      }

      return reply.send({
        synced_at: now.toISOString(),
        conflicts: [],
      });
    }
  );

  // GET /api/sync/pull
  fastify.get(
    "/api/sync/pull",
    { preHandler: [rateLimitSync] },
    async (req: AuthenticatedRequest, reply) => {
      const userId = req.user_id!;
      const query = req.query as { since?: string; device_id?: string };
      const sinceDate = query.since ? new Date(query.since) : new Date(0);
      const pulledAt = new Date().toISOString();

      const variables = await prisma.variable.findMany({
        where: {
          user_id: userId,
          updated_at: { gt: sinceDate },
        },
      });

      const dictionary = await prisma.dictionaryWord.findMany({
        where: {
          user_id: userId,
          updated_at: { gt: sinceDate },
        },
      });

      const memory = await prisma.memoryItem.findMany({
        where: {
          user_id: userId,
          updated_at: { gt: sinceDate },
        },
      });

      return reply.send({
        variables,
        dictionary,
        memory,
        pulled_at: pulledAt,
      });
    }
  );
}
