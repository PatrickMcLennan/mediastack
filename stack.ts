import * as cdk from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import * as apiGateway from '@aws-cdk/aws-apigateway';
import * as s3 from '@aws-cdk/aws-s3';
import * as path from 'path';

class AwsStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const resource = new apiGateway.RestApi(this, `aws`);
    const api = resource.root.addResource(`api`);
    const images = api.addResource(`images`);

    const bucket = new s3.Bucket(this, `aws-bucket-media-backups`, {
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
      bucketName: `aws-bucket-media-backups`
    })

    const generatePresignedUrl = new lambda.Function(this, `GeneratePresignedUrl`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(
        path.resolve(__dirname, `./lambdas/generate_presigned_url/bootstrap.zip`)
      ),
      functionName: `GeneratePresignedUrl`
    });
    api.addMethod(`GET`, new apiGateway.LambdaIntegration(generatePresignedUrl));
    bucket.grantWrite(generatePresignedUrl);

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