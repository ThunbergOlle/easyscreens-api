import { Construct } from "constructs";
import * as cdk from "aws-cdk-lib";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as iam from "aws-cdk-lib/aws-iam";
import { RetentionDays } from "aws-cdk-lib/aws-logs";

export enum HttpMethod {
  GET = "GET",
  POST = "POST",
  PUT = "PUT",
  DELETE = "DELETE",
}
export interface ServerlessLambdaEndpointProps {
  lambdaName: string;
  lambdaDescription: string;
  role: cdk.aws_iam.Role;
  method: HttpMethod;
  path: string;
}

export default class ServerlessLambdaEndpoint {
  stack: cdk.Stack;
  public lambda: cdk.aws_lambda.Function;
  public method: HttpMethod;
  public path: string;
  constructor(stack: cdk.Stack, props: ServerlessLambdaEndpointProps) {
    this.path = props.path;
    this.method = props.method;
    this.stack = stack;
    this.lambda = new lambda.Function(stack, props.lambdaName, {
      description: props.lambdaDescription,
      code: lambda.Code.fromAsset(
        `resources/target/aarch64-unknown-linux-gnu/release/lambda/lambda_${props.lambdaName}.zip`
      ),
      runtime: lambda.Runtime.PROVIDED_AL2,
      architecture: lambda.Architecture.ARM_64,
      role: props.role,
      handler: "not.required",
      environment: {
        RUST_BACKTRACE: "1",
      },
      logRetention: RetentionDays.ONE_DAY,
    });
    // add permissions to the lambda

    this.lambda.addPermission("InvokePermission", {
      principal: new iam.ServicePrincipal("apigateway.amazonaws.com"),
      sourceArn: `arn:aws:execute-api:${stack.region}:${stack.account}:*`,
    });
  }
}
