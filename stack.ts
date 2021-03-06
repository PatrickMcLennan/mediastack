import * as apigateway from '@aws-cdk/aws-apigateway';
import * as cdk from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import * as s3 from '@aws-cdk/aws-s3';
import * as dynamo from '@aws-cdk/aws-dynamodb';
import * as path from 'path';
import * as sqs from '@aws-cdk/aws-sqs';
import * as events from '@aws-cdk/aws-events';
import * as targets from '@aws-cdk/aws-events-targets';
import { DynamoEventSource, SqsEventSource } from '@aws-cdk/aws-lambda-event-sources';
import { config } from 'dotenv';

config({ path: path.resolve(__dirname, `.env`) });

class MediaStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // APIGateway
    const restApi = new apigateway.RestApi(this, 'media-apigateway', {
      restApiName: `media-apigateway`,
      defaultCorsPreflightOptions: {
        allowCredentials: true,
        allowHeaders: ['*'],
        allowOrigins: apigateway.Cors.ALL_ORIGINS,
        allowMethods: apigateway.Cors.ALL_METHODS,
      },
      description: `Rest API for Mediastack`,
    });
    const restApiPlan = restApi.addUsagePlan(`media-apigateway-usagePlan`, {
      name: `Easy`,
      description: `Only allow api key carriers access to media-apigateway`,
    });
    restApiPlan.addApiStage({
      stage: restApi.deploymentStage,
    });
    const restApiKey = new apigateway.ApiKey(this, `media-apigateway-apikey`, {
      apiKeyName: `media-apigateway-apikey`,
      description: `The API key for media-apigateway`,
      value: process.env.API_GATEWAY_API_KEY,
    });
    restApiPlan.addApiKey(restApiKey);

    // Cron
    const midnightCronJob = new events.Rule(this, 'MidnightCronJob', {
      schedule: events.Schedule.cron({ minute: `0`, hour: `0` }),
    });

    // DynamoDB
    const table = new dynamo.Table(this, `media-dynamo`, {
      tableName: `media-dynamo`,
      partitionKey: { name: 'pk', type: dynamo.AttributeType.STRING },
      sortKey: { name: 'sk', type: dynamo.AttributeType.STRING },
      stream: dynamo.StreamViewType.NEW_IMAGE,
    });
    table.addGlobalSecondaryIndex({
      indexName: `media-dynamo-index`,
      partitionKey: { name: `media_type`, type: dynamo.AttributeType.STRING },
    });

    // SQS
    const imageQueue = new sqs.Queue(this, `media-sqs-images`, {
      queueName: `media-sqs-images`,
      visibilityTimeout: cdk.Duration.seconds(500),
    });

    // S3
    const bucket = new s3.Bucket(this, `media-s3-patrick`, {
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
      bucketName: `media-s3-patrick`,
    });

    // Lambdas
    const addImageDownloadToQueue = new lambda.Function(this, `AddImageDownloadToQueue`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/add_image_download_to_queue/bootstrap.zip`)),
      functionName: `AddImageDownloadToQueue`,
    });
    addImageDownloadToQueue.addEventSource(
      new DynamoEventSource(table, {
        startingPosition: lambda.StartingPosition.LATEST,
      })
    );

    const httpGetWidescreenWallpapers = new lambda.Function(this, `HttpGetWidescreenWallpapers`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/http_get_widescreen_wallpapers/bootstrap.zip`)),
      functionName: `HttpGetWidescreenWallpapers`,
    });

    const httpGrantApiKey = new lambda.Function(this, `HttpGrantApiKey`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/http_grant_api_key/bootstrap.zip`)),
      functionName: `HttpGrantApiKey`,
    });

    const streamImageToS3 = new lambda.Function(this, `StreamImageToS3`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/stream_image_to_s3/bootstrap.zip`)),
      functionName: `StreamImageToS3`,
      timeout: cdk.Duration.seconds(500),
    });
    streamImageToS3.addEventSource(
      new SqsEventSource(imageQueue, {
        batchSize: 1,
      })
    );

    const writeWidescreenWallpapersToDynamo = new lambda.Function(this, `WriteWidescreenWallpapersToDynamo`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(
        path.resolve(__dirname, `./lambdas/write_widescreen_wallpapers_to_dynamo/bootstrap.zip`)
      ),
      functionName: `WriteWidescreenWallpapersToDynamo`,
    });
    midnightCronJob.addTarget(new targets.LambdaFunction(writeWidescreenWallpapersToDynamo));

    // HTTP Routes
    const root = restApi.root.addResource(`api`);
    const authApi = root.addResource(`auth`);
    const widescreenWallpapersApi = root.addResource(`widescreen_wallpapers`);

    authApi.addMethod(`POST`, new apigateway.LambdaIntegration(httpGrantApiKey), {
      apiKeyRequired: false,
    });

    widescreenWallpapersApi.addMethod(`GET`, new apigateway.LambdaIntegration(httpGetWidescreenWallpapers), {
      apiKeyRequired: true,
    });

    // Permissions
    table.grantReadWriteData(writeWidescreenWallpapersToDynamo);
    table.grantReadData(httpGetWidescreenWallpapers);
    imageQueue.grantSendMessages(addImageDownloadToQueue);
    imageQueue.grantConsumeMessages(streamImageToS3);
    bucket.grantWrite(streamImageToS3);
  }
}

const app = new cdk.App();
new MediaStack(app, 'MediaStack', {});
