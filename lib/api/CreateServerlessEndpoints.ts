import * as cdk from "aws-cdk-lib";

import ServerlessLambdaEndpoint, {
  HttpMethod,
} from "./ServerlessLambdaEndpoint";

export function createServerlessEndpoints(
  stack: cdk.Stack,
  role: cdk.aws_iam.Role
): ServerlessLambdaEndpoint[] {
  return [
    new ServerlessLambdaEndpoint(stack, {
      lambdaName: "get_screen",
      lambdaDescription: "Gets a screen to the database",
      role: role,
      method: HttpMethod.GET,
      path: "screen",
    }),
  ];
}
