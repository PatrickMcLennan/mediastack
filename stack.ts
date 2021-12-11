import * as cdk from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import * as apiGateway from '@aws-cdk/aws-apigateway';
import * as s3 from '@aws-cdk/aws-s3';
// import * as sqs from '@aws-cdk/aws-sqs';
import * as dynamo from '@aws-cdk/aws-dynamodb';
import * as path from 'path';
// import * as lambdaEventSources from '@aws-cdk/aws-lambda-event-sources';

class AwsStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // ApiGateway
    const gateway = new apiGateway.RestApi(this, `aws`);
    const api = gateway.root.addResource(`api`);
    const images = api.addResource(`images`);

    // DynamoDB
    const table = new dynamo.Table(this, `aws-dynamo-media`, {
      tableName: `aws-dynamo-media`,
      partitionKey: { name: 'pk', type: dynamo.AttributeType.STRING },
      sortKey: { name: 'sk', type: dynamo.AttributeType.STRING },
    });

    // S3
    const bucket = new s3.Bucket(this, `aws-bucket-media-backups`, {
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
      bucketName: `aws-bucket-media-backups`,
    });

    // SQS
    // const imageQueue = new sqs.Queue(this, `Image Queue`, {
    //   queueName: `Image Queue`,
    // });

    // Lambdas
    const generatePresignedUrl = new lambda.Function(this, `GeneratePresignedUrl`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/generate_presigned_url/bootstrap.zip`)),
      functionName: `GeneratePresignedUrl`,
    });

    const widescreenWallpapers = new lambda.Function(this, `WidescreenWallpapers`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/widescreen_wallpapers/bootstrap.zip`)),
      functionName: `WidescreenWallpapers`,
    });

    // Permissions
    api.addMethod(`GET`, new apiGateway.LambdaIntegration(generatePresignedUrl));
    bucket.grantWrite(generatePresignedUrl);
    images.addMethod(`GET`, new apiGateway.LambdaIntegration(widescreenWallpapers));
    table.grantReadWriteData(widescreenWallpapers);

    // Queues
  }
}

const app = new cdk.App();
new AwsStack(app, 'AwsStack', {});
