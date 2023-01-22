import * as cdk from "aws-cdk-lib";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import * as ssm from "aws-cdk-lib/aws-ssm";
import * as iam from "aws-cdk-lib/aws-iam";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import { Construct } from "constructs";

import { createServerlessEndpoints } from "./api/CreateServerlessEndpoints";
import ServerlessLambdaEndpoint from "./api/ServerlessLambdaEndpoint";

// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class EasyscreensStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // setup DynamoDB table and store ARN in SSM
    const screensTable = new dynamodb.Table(this, "screens", {
      partitionKey: { name: "id", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    new cdk.CfnOutput(this, "Screens Table Arn", {
      value: screensTable.tableArn,
    });

    const screensTableParam = new ssm.StringParameter(this, "screensTableArn", {
      parameterName: "/tables/screens",
      stringValue: screensTable.tableName,
    });
    // role for lambda
    const lambdaRole = new iam.Role(this, "lambdaRole", {
      assumedBy: new iam.ServicePrincipal("lambda.amazonaws.com"),
    });

    // let lambdas access screensTableParams
    screensTableParam.grantRead(lambdaRole);
    screensTable.grantFullAccess(lambdaRole);

    const endpoints: ServerlessLambdaEndpoint[] = createServerlessEndpoints(this, lambdaRole);

    // add api gateway
    const api = new apigateway.RestApi(this, "easyscreens-api", {
      restApiName: "Easyscreens API",
      description: "This service serves the easyscreens app.",
    });

    for (const endpoint of endpoints) {
      const resource = api.root.addResource(endpoint.path);
      resource.addMethod(endpoint.method, new apigateway.LambdaIntegration(endpoint.lambda));
    }
    // log the api
    new cdk.CfnOutput(this, "API URL", {
      value: api.url,
    });
    
    
  }
}
