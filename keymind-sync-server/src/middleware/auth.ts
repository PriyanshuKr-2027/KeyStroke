import { FastifyRequest, FastifyReply } from "fastify";
import jwt from "jsonwebtoken";

export interface AuthenticatedRequest extends FastifyRequest {
  user_id?: string;
  user_email?: string;
}

export async function verifySupabaseToken(
  req: AuthenticatedRequest,
  reply: FastifyReply
) {
  if (req.url.startsWith("/api/")) {
    const authHeader = req.headers.authorization;
    if (!authHeader || !authHeader.startsWith("Bearer ")) {
      reply.status(401).send({ error: "Missing or invalid authorization header" });
      return;
    }

    const token = authHeader.split(" ")[1];
    const jwtSecret = process.env.SUPABASE_JWT_SECRET || "super-secret-jwt-key";

    try {
      const decoded = jwt.verify(token, jwtSecret) as {
        sub: string;
        email?: string;
      };

      req.user_id = decoded.sub;
      req.user_email = decoded.email;
    } catch (err) {
      reply.status(401).send({ error: "Unauthorized: Invalid Supabase JWT token" });
      return;
    }
  }
}
