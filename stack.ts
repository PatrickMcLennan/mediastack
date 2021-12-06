import * as cdk from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import * as apiGateway from '@aws-cdk/aws-apigateway';
import * as path from 'path';

class AwsStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const resource = new apiGateway.RestApi(this, `aws`);
    const api = resource.root.addResource(`api`);
    const images = api.addResource(`images`);

    const widescreenWallpapers = new lambda.Function(this, `WidescreenWallpapers`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(
        path.resolve(__dirname, `./lambdas/widescreen_wallpapers/bootstrap.zip`)
      ),
      functionName: `WidescreenWallpapers`
    });
    images.addMethod(`GET`, new apiGateway.LambdaIntegration(widescreenWallpapers));
  }
}

const app = new cdk.App();
new AwsStack(app, 'AwsStack', {});