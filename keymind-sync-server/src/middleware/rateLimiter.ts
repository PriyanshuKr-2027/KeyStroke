import { FastifyReply } from "fastify";
import Redis from "ioredis";
import { AuthenticatedRequest } from "./auth";

const redis = new Redis(process.env.REDIS_URL || "redis://localhost:6379");

export async function rateLimitSync(
  req: AuthenticatedRequest,
  reply: FastifyReply
) {
  const userId = req.user_id;
  if (!userId) return;

  const key = `ratelimit:sync:${userId}`;
  const limit = 100;
  const windowSecs = 3600; // 1 hour

  const current = await redis.incr(key);
  if (current === 1) {
    await redis.expire(key, windowSecs);
  }

  if (current > limit) {
    reply.status(429).send({
      error: "Too Many Requests: Rate limit exceeded (100 sync requests per hour)",
    });
    return;
  }
}

export async function rateLimitExport(
  req: AuthenticatedRequest,
  reply: FastifyReply
) {
  const userId = req.user_id;
  if (!userId) return;

  const key = `ratelimit:export:${userId}`;
  const limit = 10;
  const windowSecs = 86400; // 24 hours

  const current = await redis.incr(key);
  if (current === 1) {
    await redis.expire(key, windowSecs);
  }

  if (current > limit) {
    reply.status(429).send({
      error: "Too Many Requests: Rate limit exceeded (10 export requests per day)",
    });
    return;
  }
}
