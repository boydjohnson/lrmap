resource "aws_elasticache_cluster" "redis-lrmap" {
  cluster_id           = "redis-lrmap"
  engine               = "redis"
  node_type            = "cache.t4g.micro"
  num_cache_nodes      = 1
  parameter_group_name = "default.redis6.x"
  engine_version       = "6.2"
  port                 = 6379
  subnet_group_name    = aws_elasticache_subnet_group.subnet-group-lrmap-redis.name
}
