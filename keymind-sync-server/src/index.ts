import Fastify from "fastify";
import dotenv from "dotenv";
import { verifySupabaseToken } from "./middleware/auth";
import { syncRoutes } from "./routes/sync";
import { deviceRoutes } from "./routes/devices";
import { accountRoutes } from "./routes/account";

dotenv.config();

const fastify = Fastify({ logger: true });

// Register Auth Hook for /api/* routes
fastify.addHook("onRequest", verifySupabaseToken);

// Health check endpoint
fastify.get("/health", async () => {
  return { status: "ok", service: "keymind-sync-server", timestamp: new Date() };
});

// Register routes
fastify.register(syncRoutes);
fastify.register(deviceRoutes);
fastify.register(accountRoutes);

const PORT = parseInt(process.env.PORT || "3000", 10);
const HOST = process.env.HOST || "0.0.0.0";

const start = async () => {
  try {
    await fastify.listen({ port: PORT, host: HOST });
    fastify.log.info(`KeyMind Cloud Sync API server listening on ${HOST}:${PORT}`);
  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
};

start();
